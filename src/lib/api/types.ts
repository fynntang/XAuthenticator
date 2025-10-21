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