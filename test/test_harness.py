from test.test_schema import Interface, Key, Peer
from test.test_settings import URL
import requests
from typing import Dict
import pytest
import random


def process_ifc_json(ifc_json: Dict) -> Interface:
    peers = []
    for p_json in ifc_json["peers"]:
        peers.append(Peer(public_key=p_json["public_key"],
                          allowed_ips=p_json["allowed_ips"],
                          persistent_keepalive=p_json["persistent_keepalive"],
                          endpoint=p_json["endpoint"]))
    return Interface(name=ifc_json["name"],
                     public_key=ifc_json["public_key"],
                     private_key=ifc_json["private_key"],
                     listen_port=ifc_json["listen_port"],
                     address=ifc_json["address"],
                     peers=peers)


def create_interface(
    ifc_name: str = "test0",
    address: str = "192.0.2.2/32",
    listen_port: int = 51111,
    set_link_up: bool = False,
    persist: bool = False
) -> Interface:
    r = requests.post(
        f"{URL}/wg/interface",
        json={"ifc_name": ifc_name,
              "address": address,
              "listen_port": listen_port,
              "set_link_up": set_link_up,
              "persist": persist})
    return process_ifc_json(r.json())


def delete_interface(ifc_name: str) -> bool:
    r = requests.delete(
        f"{URL}/wg/interface/{ifc_name}"
    )
    print(f"delete ifc result={r}")
    return r.ok


@pytest.fixture()
def make_interface():
    interface = create_interface(ifc_name="test123")
    yield interface
    delete_interface(ifc_name="test123")


def gen_private_key() -> str:
    r = requests.get(f"{URL}/wg/genkey")
    private = r.json()["key"]
    return private


def gen_psk() -> str:
    r = requests.get(f"{URL}/wg/gepsk")
    psk = r.json()["key"]
    return psk


def get_private_key(ifc_name: str) -> str:
    r = requests.get(f"{URL}/wg/show/{ifc_name}/private-key")
    res = r.json()
    return res["private_key"]


def get_listen_port(ifc_name: str) -> int:
    r = requests.get(f"{URL}/wg/show/{ifc_name}/listen-port")
    res = r.json()
    return res["listen_port"]


def get_fwmark(ifc_name: str) -> str:
    r = requests.get(f"{URL}/wg/show/{ifc_name}/fwmark")
    res = r.json()
    return res["fwmark"]


def get_public_key(private_key: str) -> Key:
    r = requests.post(f"{URL}/wg/pubkey",
                      json={
                          "key": private_key
                      })
    return Key(key=r.json()["key"])


def gen_endpoint() -> str:
    return f"198.51.100.{random.randrange(2,254,1)}:{random.randrange(49152,65535,1)}"


def gen_fake_peer() -> Peer:
    private_key = gen_private_key()
    public_key = get_public_key(private_key).key
    ip = f"198.51.100.{random.randrange(1,254,1)}"
    endpoint = f"{ip}:{random.randrange(49152,65535,1)}"
    persistent_keepalive = random.randrange(5, 60, 1)
    allowed_ips = f"{ip}/32"

    return Peer(private_key, public_key, endpoint,
                persistent_keepalive, allowed_ips)


def add_peer(ifc_name: str, peer: Peer) -> Interface:
    r = requests.post(f"{URL}/wg/interface/{ifc_name}/peer",
                      json={
                          "public_key": peer.public_key,
                          "endpoint": peer.endpoint,
                          "allowed_ips": peer.allowed_ips,
                          "persistent_keepalive": peer.persistent_keepalive,
                      })
    result = r.json()
    print(f"add peer result json={result}")
    return process_ifc_json(result)


def remove_peer(ifc_name: str, peer: str) -> Interface:
    r = requests.delete(f"{URL}/wg/interface/{ifc_name}/peer",
                        json={
                            "key": peer
                        })
    return process_ifc_json(r.json())


def test_wg_show():
    """
    {
    "interfaces": [
        {
        "name": "wg2",
        "public_key": "DnF7Mwjbz8Whmb+tX8rCHgqYrakEpeGZVMtXfHOHVm0=",
        "private_key": "",
        "listen_port": 54324,
        "address": "",
        "peers": [
            {
            "public_key": "qck6YLr1mk2XzCYP6MAHf+Ynb8wPNjcrlWVyb2Rppwk=",
            "allowed_ips": "0.0.0.0/0",
            "persistent_keepalive": "every 10 seconds",
            "endpoint": "34.219.225.131:54322"
            }
        ]
        }
    ]
    }
    """
    r = requests.get(f"{URL}/wg/show")
    # should return 200
    assert r.ok
    # should contain JSON
    result = r.json()
    # JSON result should contain an "interfaces" field
    interfaces = result.get("interfaces", None)
    assert interfaces is not None


def test_wg_show_interfaces():
    """
    {
    "interfaces": [
        "wg2",
        "wg0",
        "wg1"
    ]
    }
    """
    r = requests.get(f"{URL}/wg/show/interfaces")
    assert r.ok
    result = r.json()
    interfaces = result.get("interfaces", None)
    assert interfaces is not None


def test_create_delete_wg_interface():
    interface: Interface = create_interface(ifc_name="test123")
    assert interface.name == "test123"
    assert delete_interface(ifc_name="test123") is True


def test_wg_show_interface(make_interface):
    ifc_name = make_interface.name
    r = requests.get(f"{URL}/wg/show/{ifc_name}")
    assert r.ok
    result = r.json()
    # {'name': 'test123', 'public_key': 'Ah6OUzygAvG78nIyMlN8+tIOeZmKQ2prKWtK6i7KgyI=', 'private_key': '', 'listen_port': 51111, 'address': '', 'peers': []}
    print(f"result_json={result}")
    assert result["name"] == "test123"


def test_wg_show_public_key(make_interface):
    ifc_name = make_interface.name
    exp_public_key = make_interface.public_key
    r = requests.get(f"{URL}/wg/show/{ifc_name}/public-key")
    result = r.json()
    assert result["public_key"] == exp_public_key


def test_wg_show_private_key(make_interface):
    ifc_name = make_interface.name
    exp_private_key = make_interface.private_key
    r = requests.get(f"{URL}/wg/show/{ifc_name}/private-key")
    result = r.json()
    assert result["private_key"] == exp_private_key


def test_wg_show_listen_port(make_interface):
    ifc_name = make_interface.name
    exp_listen_port = make_interface.listen_port
    r = requests.get(f"{URL}/wg/show/{ifc_name}/listen-port")
    result = r.json()
    assert result["listen_port"] == exp_listen_port


def test_wg_show_fwmark(make_interface):
    ifc_name = make_interface.name
    exp_fwmark = "off"
    r = requests.get(f"{URL}/wg/show/{ifc_name}/fwmark")
    result = r.json()
    assert result["fwmark"] == exp_fwmark


def test_wg_show_peers(make_interface):
    ifc_name = make_interface.name
    peers = []
    r = requests.get(f"{URL}/wg/show/{ifc_name}/peers")
    result = r.json()
    assert len(result["peers"]) == len(peers)


def test_wg_show_endpoints(make_interface):
    ifc_name = make_interface.name
    endpoints = []
    r = requests.get(f"{URL}/wg/show/{ifc_name}/endpoints")
    result = r.json()
    assert len(result["endpoints"]) == len(endpoints)


def test_wg_show_allowed_ips(make_interface):
    ifc_name = make_interface.name
    allowed_ips = []
    r = requests.get(f"{URL}/wg/show/{ifc_name}/allowed-ips")
    result = r.json()
    assert len(result["allowed_ips"]) == len(allowed_ips)


def test_wg_show_latest_handshakes(make_interface):
    ifc_name = make_interface.name
    latest_handshakes = []
    r = requests.get(f"{URL}/wg/show/{ifc_name}/latest-handshakes")
    result = r.json()
    assert len(result["latest_handshakes"]) == len(latest_handshakes)


def test_wg_show_persistent_keepalive(make_interface):
    ifc_name = make_interface.name
    persistent_keepalives = []
    r = requests.get(f"{URL}/wg/show/{ifc_name}/persistent-keepalive")
    result = r.json()
    assert len(result["persistent_keepalives"]) == len(persistent_keepalives)


def test_wg_show_transfer(make_interface):
    ifc_name = make_interface.name
    transfers = []
    r = requests.get(f"{URL}/wg/show/{ifc_name}/transfer")
    result = r.json()
    assert len(result["transfers"]) == len(transfers)


def test_wg_showconf(make_interface):
    ifc_name = make_interface.name
    r = requests.get(f"{URL}/wg/showconf/{ifc_name}")
    file_content = r.text
    print(f"downloaded conf:\n{file_content}\n")
    # [Interface]
    # ListenPort = 51111
    # PrivateKey = 6M0wLVKY7+/s7NC/2szmL/ARi/vk6uZrLr5Xt70VbmQ=
    assert "[Interface]" in file_content
    assert "ListenPort = " in file_content
    assert "PrivateKey = " in file_content


def test_wg_genkey():
    r = requests.get(f"{URL}/wg/genkey")
    private = r.json()["key"]
    assert len(private) > 0


def test_wg_genpsk():
    r = requests.get(f"{URL}/wg/genpsk")
    psk = r.json()["key"]
    assert len(psk) > 0


def test_wg_pubkey():
    private_key = gen_private_key()
    r = requests.post(f"{URL}/wg/pubkey", json={"key": private_key})
    result = r.json()
    pub_key = result["key"]
    assert len(pub_key) > 0


def test_wg_set_private_key(make_interface):
    ifc_name = make_interface.name
    pre_private_key = make_interface.private_key
    new_private_key = gen_private_key()
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "private_key": new_private_key
                     })
    assert r.ok
    post_private_key = get_private_key(ifc_name)
    assert new_private_key == post_private_key
    assert pre_private_key != post_private_key


def test_wg_set_listen_port(make_interface):
    ifc_name = make_interface.name
    pre_listen_port = make_interface.listen_port
    new_listen_port = 52222
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "listen_port": new_listen_port
                     })
    assert r.ok
    post_listen_port = get_listen_port(ifc_name)
    assert new_listen_port == post_listen_port
    assert pre_listen_port != post_listen_port


def test_wg_set_fwmark(make_interface):
    ifc_name = make_interface.name
    pre_fwmark = "off"
    new_fwmark = "0x12345678"
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "fwmark": new_fwmark
                     })
    assert r.ok
    post_fwmark = get_fwmark(ifc_name)
    assert new_fwmark == post_fwmark
    assert pre_fwmark != post_fwmark


def test_wg_add_remove_peer(make_interface):
    fake_peer: Peer = gen_fake_peer()
    ifc_name = make_interface.name
    pre_interface: Interface = add_peer(ifc_name, fake_peer)
    print(f"pre_interface={pre_interface}")
    assert len(pre_interface.peers) > 0

    post_interface: Interface = remove_peer(ifc_name, fake_peer.public_key)
    assert len(post_interface.peers) == 0


def test_wg_set_peer_remove(make_interface):
    fake_peer: Peer = gen_fake_peer()
    ifc_name = make_interface.name
    add_peer(ifc_name, fake_peer)
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "peer": {
                             "public_key": fake_peer.public_key,
                             "remove": True,
                         }
                     })
    post_interface = process_ifc_json(r.json())
    assert len(post_interface.peers) == 0


def test_wg_set_peer_endpoint(make_interface):
    fake_peer: Peer = gen_fake_peer()
    ifc_name = make_interface.name
    pre_interface: Interface = add_peer(ifc_name, fake_peer)
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "peer": {
                             "public_key": fake_peer.public_key,
                             "endpoint": gen_endpoint()
                         }
                     })
    post_ifc = process_ifc_json(r.json())
    assert post_ifc.peers[0].endpoint != pre_interface.peers[0].endpoint


def test_wg_set_peer_allowed_ips(make_interface):
    fake_peer: Peer = gen_fake_peer()
    ifc_name = make_interface.name
    pre_ifc: Interface = add_peer(ifc_name, fake_peer)
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "peer": {
                             "public_key": fake_peer.public_key,
                             "allowed_ips": f"{fake_peer.allowed_ips},10.239.239.0/24"
                         }
                     })
    post_ifc = process_ifc_json(r.json())
    assert post_ifc.peers[0].allowed_ips != pre_ifc.peers[0].allowed_ips


def test_wg_set_peer_persistent_keepalive(make_interface):
    fake_peer: Peer = gen_fake_peer()
    ifc_name = make_interface.name
    pre_ifc: Interface = add_peer(ifc_name, fake_peer)
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "peer": {
                             "public_key": fake_peer.public_key,
                             "persistent_keepalive": 123,
                         }
                     })
    post_ifc = process_ifc_json(r.json())
    assert post_ifc.peers[0].persistent_keepalive != pre_ifc.peers[0].persistent_keepalive


def test_wg_set_peer_psk(make_interface):
    fake_peer: Peer = gen_fake_peer()
    ifc_name = make_interface.name
    pre_ifc: Interface = add_peer(ifc_name, fake_peer)
    psk = gen_psk()
    r = requests.put(f"{URL}/wg/set/{ifc_name}",
                     json={
                         "peer": {
                             "public_key": fake_peer.public_key,
                             "preshared_key": psk,
                         }
                     })
    post_ifc = process_ifc_json(r.json())
    assert post_ifc.peers[0].preshared_key == psk


def test_wg_setconf():
    assert False


def test_wg_addconf():
    assert False


def test_wg_syncconf():
    assert False


def test_wg_import_config():
    assert False


# END OF FILE
