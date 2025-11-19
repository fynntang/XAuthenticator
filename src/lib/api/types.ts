import {CommonError} from "$lib/api/errors";


export type AppDefault = {
    kdbxPath: string
}

export type InitRequest = {
    kdbxPath: string,
    password: string,
}

export type APIError = {
    code: CommonError,
    reason: string,
}

export type AppStateResponse = {
    isInitialized: boolean,
    runtimeTimestamp: number | null,
    isLocked: boolean,
    lockedTimestamp: number | null,
    config: {
        path: string,
        builder: {
            kdbxPath: string,
            settings: {
                theme: string,
                language: string,
                autoLock: boolean,
                autoLockTimeout: number
            }
        }
    }
}

export type Account = {};