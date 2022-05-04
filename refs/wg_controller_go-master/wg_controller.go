package main

import (
	"bufio"
	"bytes"
	"errors"
	"fmt"
	"io"
	"ioutil"
	"net/http"
	"os"
	"os/exec"
	"strconv"

	"text/template"

	"github.com/apex/log"
	"github.com/apex/log/handlers/text"
	"github.com/gin-gonic/gin"
	"gopkg.in/alecthomas/kingpin.v2"
)

// func createMyRender() multitemplate.Renderer {
// 	r := multitemplate.NewRenderer()
// }

const defaultIfcListenPort = 51820

const defaultInterfaceIP = "10.255.255.1/24"

const interfaceConfigTemplate = `
[Interface]
Address = {{.ServerVirtualIP}}
ListenPort = {{.ServerListenPort}}
PrivateKey = {{.ServerPrivateKey}}
{{if .IncludeDNS}}
DNS = {{.ServerDNSAddress}}
{{- else}}
{{- end}}
{{if .SetTableOff}}
Table = off
{{- else}}
{{if .SetTableValue}}
Table = {{.RoutingTable}}
{{- else}}
Table = auto
{{- end}}
{{- end}}
{{if .SetPreUp}}
PreUp = {{.ServerPreUp}}
{{- end}}
{{if .SetPostUp}}
PostUp = {{.ServerPostUp}}
{{- end}}
{{if .SetPreDown}}
PreDown = {{.ServerPreDown}}
{{- end}}
{{if .SetPostDown}}
PostDown = {{.ServerPostDown}}
{{- end}}
`

type InterfaceConfig struct {
	ServerVirtualIP  string
	ServerListenPort uint16
	ServerPrivateKey string
	IncludeDNS       bool
	ServerDNSAddress string
	SetTableOff      bool
	SetTableValue    bool
	RoutingTable     uint32
	SetPreUp         bool
	ServerPreUp      string
	SetPreDown       bool
	ServerPreDown    string
	SetPostUp        bool
	ServerPostUp     string
	SetPostDown      bool
	ServerPostDown   string
}

var (
	endpointAddress   = kingpin.Flag("endpoint", "Address for peers to connect to.").String()
	controllerPort    = kingpin.Flag("controller_port", "port for controller to listen on").Default("8100").String()
	controllerAddress = kingpin.Flag("controller_addr", "address for the controller to listen on").Default("").String()
)

func genInterfaceConf(privateKey string, ifcIP string, ifcMask string, listenPort uint16) ([]byte, error) {
	t := template.Must(template.New("interfaceConfigTemplate").Parse(interfaceConfigTemplate))

	ip := fmt.Sprintf("%s/%s", ifcIP, ifcMask)

	cfg := InterfaceConfig{
		ip,
		listenPort,
		privateKey,
		false,
		"",
		true,
		false,
		0,
		false,
		"",
		false,
		"",
		false,
		"",
		false,
		"",
	}

	var buff bytes.Buffer
	if err := t.Execute(&buff, cfg); err != nil {
		log.Errorf("failed to generate data with template: %s", err)
		return nil, errors.New("failed to generate data with template")
	}
	log.Debugf("output:\n%s\n", buff.String())
	return buff.Bytes(), nil
}

func genPrivateKey() ([]byte, error) {
	log.Info("generating private key")
	out, err := exec.Command("sudo", "wg", "genkey").Output()
	if err != nil {
		log.Errorf("failed to generate private key: %s", err)
		return nil, errors.New("failed to generate private key")
	}
	log.Debugf("wg genkey: %s\n", out)
	return out, nil
}

func genPublicKey(privateKey []byte) ([]byte, error) {
	cmd := exec.Command("sudo", "wg", "pubkey")
	stdin, err := cmd.StdinPipe()
	if err != nil {
		log.Errorf("failed to get stdin pipe: %s", err)
		return nil, errors.New("failed to get stdin pipe")
	}

	go func() {
		defer stdin.Close()
		io.WriteString(stdin, string(privateKey))
	}()

	out, err := cmd.CombinedOutput()
	if err != nil {
		log.Errorf("failed to run command: %s", err)
		return nil, errors.New("failed to generate public key")
	}
	log.Infof("wg pubkey: %s\n", out)
	return out, nil
}

func createInterface(ifcName string, ifcIP string, ifcMask string, listenPort string) (error) {
	privateKey, err := genPrivateKey()
	if err != nil {
		log.Errorf("failed to generate private key: %s", err)
		return errors.New("failed to generate private key")
	}

	// save private key to file
	privateKeyFileName := fmt.Sprintf("%s_private.key", ifcName)
	privateKeyTmpPath := fmt.Sprintf("/tmp/%s", privateKeyFileName)
	privateKeyWgPath := fmt.Sprintf("/etc/wg/%s", privateKeyFileName)

	fd, err := os.Create(privateKeyTmpPath)
	if err != nil {
		log.Errorf("failed to open temp private key file for writing: %s", err)
		return errors.New("failed to open temp private key file for writing")
	}
	defer fd.Close()

	numWritten, err := fd.Write(privateKey)
	if err != nil {
		log.Errorf("failed to write private key to temp file: %s", err)
		return errors.New("failed to write private key to temp file")
	} 
	log.Debugf("%d bytes written to file", numWritten)

	// copy private key to wireguard dir
	out, err := exec.Command("sudo", "cp", privateKeyTmpPath, privateKeyWgPath).Output()
	if err != nil {
		log.Errorf("failed to copy private key to wireguard dir: %s", err)
		return errors.New("failed to copy private key to wireguard dir")
	}
	log.Debugf("copy file result: %s\n", out)

	publicKey, err := genPublicKey(privateKey)
	if err != nil {
		log.Errorf("failed to generate public key: %s", err)
		return errors.New("failed to generate public key")
	}

	// save public key to file
	publicKeyFileName := fmt.Sprintf("%s_public.key", ifcName)
	publicKeyTmpPath := fmt.Sprintf("/tmp/%s", publicKeyFileName)
	publicKeyWgPath := fmt.Sprintf("/etc/wg/%s", publicKeyFileName)

	fd2, err := os.Create(publicKeyTmpPath)
	if err != nil {
		log.Errorf("failed to open temp public key file for writing: %s", err)
		return errors.New("failed to open temp public key file for writing")
	}
	defer fd2.Close()

	numWritten, err = fd2.Write(publicKey)
	if err != nil {
		log.Errorf("failed to write public key to temp file: %s", err)
		return errors.New("failed to write public key to temp file")
	} 
	log.Debugf("%d bytes written to file", numWritten)
	
	out, err = exec.Command("sudo", "cp", publicKeyTmpPath, publicKeyWgPath).Output()
	if err != nil {
		log.Errorf("failed to copy public key to wireguard dir: %s", err)
		return errors.New("failed to copy public key to wireguard dir")
	}
	log.Debugf("copy file result: %s\n", out)

	portU64, err := strconv.ParseInt(listenPort, 10, 16)
	if err != nil {
		log.Errorf("failed to convert listen port from string to Uint64: %s", err)
		return errors.New("failed to convert listen port from string to integer")
	}

	portU16 := uint16(portU64)

	ifcCfg, err := genInterfaceConf(string(privateKey), ifcIP, ifcMask, portU16)
	if err != nil {
		log.Errorf("failed to generate interface config: %s", err)
		return errors.New("failed to generate interface config")
	}

	ifcCfgFilename := fmt.Sprintf("%s.conf", ifcName)
	ifcCfgTmpPath := fmt.Sprintf("/tmp/%s", ifcCfgFilename)
	ifcCfgWgPath := fmt.Sprintf("/etc/wg/%s", ifcCfgFilename)
	
	fd3, err := os.Create(ifcCfgTmpPath)
	if err != nil {
		log.Errorf("failed open temp ifc cfg file for writing: %s", err)
		return errors.New("failed to open temp ifc cfg file for writing")
	}
	defer fd3.Close()

	numWritten, err = fd3.Write(ifcCfg)
	if err != nil {
		log.Errorf("failed to write ifc cfg to tmp ifc cfg file: %s", err)
		return errors.New("faild to write ifc cfg to tmp ifc cfg file")
	}
	log.Debugf("num bytes written to file: %d", numWritten)

	out, err = exec.Command("sudo", "cp", ifcCfgTmpPath, ifcCfgWgPath).Output()
	if err != nil {
		log.Errorf("failed to copy public key to wireguard dir: %s", err)
		return errors.New("failed to copy public key to wireguard dir")
	}
	log.Debugf("copy file result: %s\n", out)

	// enable the service for the interface

	

	return nil
}

func setupRouter() *gin.Engine {
	r := gin.Default()

	r.GET("/ping", func(c *gin.Context) {
		c.String(http.StatusOK, "pong")
	})

	r.GET("/interfaces", func(c *gin.Context) {
		out, err := exec.Command("sudo", "wg", "show", "interfaces").Output()
		if err != nil {
			log.Errorf("failed to get interfaces: %s", err)
			c.AbortWithStatus(400)
		}
		log.Infof("wg show interfaces: %s\n", out)
		c.String(http.StatusOK, fmt.Sprintf("interfaces: %s", out))
	})

	r.GET("/interfaces/:interface_name", func(c *gin.Context) {
		interfaceName := c.Param("interface_name")
		log.Infof("requested interface: \"%s\"", interfaceName)
		out, err := exec.Command("sudo", "wg", "show", interfaceName).Output()
		if err != nil {
			log.Errorf("failed to get interface information: %s", err)
			c.AbortWithStatus(400)
		}
		log.Infof("wg show %s: %s\n", interfaceName, out)
		c.String(http.StatusOK, fmt.Sprintf("interface: %s: %s", interfaceName, out))
	})

	r.GET("/utils/gen_key_pair", func(c *gin.Context) {
		privateKey, err := genPrivateKey()
		if err != nil {
			log.Errorf("failed to generate private key: %s", err)
			c.AbortWithStatus(400)
		}

		publicKey, err := genPublicKey(privateKey)
		if err != nil {
			log.Errorf("failed to gnerate public key: %s", err)
			c.AbortWithStatus(400)
		}

		c.String(http.StatusOK, fmt.Sprintf("private key: %s\npublic key: %s\n", privateKey, publicKey))
	})

	r.GET("/utils/gen_ifc_cfg/:interface_name", func(c *gin.Context) {
		privateKey, err := genPrivateKey()
		if err != nil {
			log.Errorf("failed to generate private key")
			c.AbortWithStatus(400)
		}

		interfaceName := c.Param("interface_name")
		interfaceIP := c.DefaultQuery("addr", defaultInterfaceIP)
		interfaceMask := c.DefaultQuery("mask", "24")
		listenPort := c.DefaultQuery("port", fmt.Sprintf("%d", defaultIfcListenPort))
		listenPort64, err := strconv.ParseUint(listenPort, 10, 16)
		if err != nil {
			log.Errorf("failed to parse listen port query parameter: %s", listenPort)
			c.AbortWithStatus(400)
		}

		var listenPort16 uint16 = uint16(listenPort64)

		ifcConfig, err := genInterfaceConf(string(privateKey), interfaceIP, interfaceMask, listenPort16)
		if err != nil {
			log.Errorf("failed to generate interface config")
			c.AbortWithStatus(400)
		}

		c.String(http.StatusOK, fmt.Sprintf("interface name: %s\ninterface config:\n%s\n", interfaceName, ifcConfig))

	})

	return r
}

func main() {
	log.SetHandler(text.New(os.Stdout))

	kingpin.Version("0.1")
	kingpin.Parse()

	log.Infof("endpoint address: %s", *endpointAddress)

	r := setupRouter()

	listenAddr := fmt.Sprintf("%s:%s", *controllerAddress, *controllerPort)
	log.Infof("starting controller server. Listening at: %s", listenAddr)
	r.Run(listenAddr)
}
