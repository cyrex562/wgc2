from test.test_settings import URL
import requests
from typing import Tuple
from requests import Response
import pytest

def create_interface(
    ifc_name: str = "test0", 
    address: str = "192.0.2.2/32", 
    listen_port: int = 51111, 
    set_link_up: bool = False, 
    persist: bool = False
    ) -> Tuple[str, Response]:
    r = requests.post(
        f"{URL}/wg/interface", 
        json={"ifc_name": ifc_name, 
        "address": address, 
        "listen_port": listen_port, 
        "set_link_up": set_link_up, 
        "persist": persist})
    print(f"create ifc result={r}")

    return r.json()

def delete_interface(ifc_name: str) -> bool:
    r = requests.delete(
        f"{URL}/wg/interface/{ifc_name}"
    )
    print(f"delete ifc result={r}")
    return r.ok


@pytest.fixture()
def make_interface():
    r_json = create_interface(ifc_name="test123")
    yield r_json
    delete_interface(ifc_name="test123")


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
    result_json = create_interface(ifc_name="test123")
    assert result_json["name"] == "test123"
    assert result_json.get("name", None) is not None
    assert delete_interface(ifc_name="test123") is True

def test_wg_show_interface(make_interface):
    ifc_name = make_interface["name"]
    r = requests.get(f"{URL}/wg/show/{ifc_name}")
    assert r.ok
    result = r.json()
    # {'name': 'test123', 'public_key': 'Ah6OUzygAvG78nIyMlN8+tIOeZmKQ2prKWtK6i7KgyI=', 'private_key': '', 'listen_port': 51111, 'address': '', 'peers': []}
    print(f"result_json={result}")
    assert result["name"] == "test123"

def test_wg_show_public_key(make_interface):
    ifc_name = make_interface["name"]
    exp_public_key = make_interface["public_key"]
    r = requests.get(f"{URL}/wg/show/{ifc_name}/public-key")
    result = r.json()
    assert result["public_key"] == exp_public_key

def test_wg_show_private_key(make_interface):
    ifc_name = make_interface["name"]
    exp_private_key = make_interface["private_key"]
    r = requests.get(f"{URL}/wg/show/{ifc_name}/private-key")
    result = r.json()
    assert result["private_key"] == exp_private_key

def test_wg_show_listen_port():
    assert False

def test_wg_show_fwmark():
    assert False

def test_wg_show_peers():
    assert False

def test_wg_show_endpoints():
    assert False

def test_wg_show_allowed_ips():
    assert False

def test_wg_show_latest_handshakes():
    assert False

def test_wg_show_persistent_keepalive():
    assert False

def test_wg_show_transfer():
    assert False

def test_wg_showconf():
    assert False

def test_wg_genkey():
    assert False

def test_wg_genpsk():
    assert False

def test_wg_pubkey():
    assert False



def test_wg_set_private_key():
    assert False

def test_wg_set_listen_port():
    assert False

def test_wg_set_fwmark():
    assert False

def test_wg_set_peer_remove():
    assert False

def test_wg_set_peer_endpoint():
    assert False

def test_wg_set_peer_allowed_ips():
    assert False

def test_wg_set_peer_persistent_keepalive():
    assert False

def test_wg_set_peer_psk():
    assert False

def test_wg_add_peer():
    assert False

def test_wg_remove_peer():
    assert False

def test_wg_setconf_peer():
    assert False

def test_wg_addconf():
    assert False

def test_wg_syncconf():
    assert False

def test_wg_import_config():
    assert False




# def check_create_interface(app_ctx: AppContext) -> bool:
#     ifc_name, response = create_interface(app_ctx)
#     print(f"interface name={ifc_name}")
#     if not response.ok():
#         print("failed to create interface")
#         return False
#     return delete_interface(app_ctx, ifc_name)


# def check_wg_set_fwmark_0(app_ctx: AppContext) -> bool:
#     return False


# def check_wg_set_fwmark_val(app_ctx: AppContext) -> bool:
#     return False


# ALL_CHECKS = {
#     "wg_set_fwmark_0": check_wg_set_fwmark_0,
#     "wg_set_fwmark_val": check_wg_set_fwmark_val
# }

# def run() -> int:
#     parser = argparse.ArgumentParser()
#     parser.add_argument("--test","-t", required=True, choices=ALL_CHECKS.keys())
#     parser.add_argument("--address","-a", required=True)
#     args = parser.parse_args()
#     test_to_run: str = args.test.lower()
    
#     if test_to_run == "all":
#         for check in ALL_CHECKS.values():
#             result = check()
#             print(f"check={check.__name__}, result={result}")
#     elif ALL_CHECKS.get(test_to_run, None) is not None:
#         check = ALL_CHECKS[test_to_run]
#         result = check()
#         print(f"check={check.__name__}, result={result}")
#     else:
#         print(f"invalid test name={test_to_run}, valid tests={ALL_CHECKS.keys()}")
#     return 0


# if __name__ == "__main__":
#     sys.exit(run())

# END OF FILE