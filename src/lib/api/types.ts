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

export type PageParam = {
    current: number,
    size: number,
};

export type Response<T> = {
    data: T,
}

export type PagedResponse<T> = {
    data: T[],
    total: number,
    current: number,
    size: number,
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

export type Account = {
    id: string,
    issuer: string,
    label: string,
    type: string,
    algorithm: string,
    digits: number,
    period?: number,
    counter?: number,
    secretCipher: number[],
    icon?: number[],
    note?: string,
    createdAt?: number,
    updatedAt?: number,
};

export type CreateAccountRequest = {
    issuer: string,
    label: string,
    type: string,
    algorithm: string,
    digits: number,
    period?: number,
    counter?: number,
    secret: string,
    icon?: number[],
    note?: string,
};

export type UpdateAccountRequest = {
    id: string,
    issuer?: string,
    label?: string,
    icon?: number[],
    note?: string,
};