from libpyaoaddons import subscribe, initialize  # type: ignore


class InitializationResult:
    Ok = 0
    UnknownFailure = 1
    NetworkInterfaceListMissing = 2


__all__ = ['subscribe', 'initialize', 'InitializationResult']
