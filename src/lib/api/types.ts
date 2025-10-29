import {CommonError} from "$lib/api/errors";

export type APIError = {
    code: CommonError,
    reason: string,
}

export type PageParam = {
    current: number,
    size: number,
};

export type Response<T> = {
    data: T,
    total: number,
}

export type AppStateResponse = {
    isInitialized: boolean,
    config: {
        path: string,
        builder: {
            settings: {
                theme: string,
                language: string,
                autoLock: boolean,
                autoLockTimeout: number
            }
        }
    }
    isLocked: boolean
}

export type Account = {
    id: string,
    issuer: string,
    label: string,
    type: string,
};