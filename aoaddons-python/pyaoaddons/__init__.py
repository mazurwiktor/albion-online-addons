from libpyaoaddons import subscribe, initialize  # type: ignore
from .item_category_mapping import mapping as item_category_mapping
from .localization_mapping import mapping as localization_mapping

class InitializationResult:
    Ok = 0
    UnknownFailure = 1
    NetworkInterfaceListMissing = 2


__all__ = ['subscribe', 'initialize', 'item_category_mapping', 'localization_mapping', 'InitializationResult']
