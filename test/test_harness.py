import requests
from typing import Tuple
from requests import Response
import pytest

class AppContext:
    def __init__(self, address):
        self.address = f"http://{address}/api/v1"

def create_interface(app_ctx: AppContext, 
    ifc_name: str = "test0", 
    address: str = "192.0.2.2/32", 
    listen_port: int = 51111, 
    set_link_up: bool = False, 
    persist: bool = False) -> Tuple[str, Response]:
    r = requests.post(
        f"{app_ctx.address}/wg/interface", 
        data={"ifc_name": ifc_name, 
        "address": address, 
        "listen_port": listen_port, 
        "set_link_up": set_link_up, 
        "persist": persist})
    print(f"create ifc result={r}")

    return ifc_name, r

def delete_interface(app_ctx: AppContext, ifc_name: str) -> bool:
    r = requests.delete(
        f"{app_ctx.address}/wg/interface/{ifc_name}"
    )
    print(f"delete ifc result={r}")
    return r.ok()

def test_wg_show():
    assert False

def test_wg_show_interfaces():
    assert False

def test_wg_show_interface():
    assert False

def test_wg_show_public_key():
    assert False

def test_wg_show_private_key():
    assert False

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

def test_create_wg_interface():
    assert False

def test_delete_wg_interface():
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