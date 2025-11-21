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

export type Times = {
    expires: boolean;
    usageCount: number;
    times: Map<string, Date>;
}
export const TimesConstants = {
    EXPIRY_TIME_TAG_NAME: 'ExpiryTime',
    LAST_MODIFICATION_TIME_TAG_NAME: 'LastModificationTime',
    CREATION_TIME_TAG_NAME: 'CreationTime',
    LAST_ACCESS_TIME_TAG_NAME: 'LastAccessTime',
    LOCATION_CHANGED_TAG_NAME: 'LocationChanged'
} as const;

// 自定义数据
export type CustomData = {
    items: Map<string, CustomDataItem>;
}
export type CustomDataItem = {
    value?: Value;
    lastModificationTime?: Date;
};

export type CustomDataItemDenormalized = {
    key: string;
    customDataItem: CustomDataItem;
}

export type Color = {
    r: number;
    g: number;
    b: number;
}

export type Value =
    | { type: 'Bytes', data: Uint8Array }
    | { type: 'Unprotected', data: string }
    | { type: 'Protected', data: string }; // 注意：在TS中我们通常不处理安全内存

export type Node =
    | { type: 'Group', group: Group }
    | { type: 'Entry', entry: Entry };

export type NodeRef<T extends Node> =
    T extends { type: 'Group' } ? Group :
        T extends { type: 'Entry' } ? Entry : never;

export type NodeIter = Iterable<NodeRef<any>>;

export type AutoType = {
    enabled: boolean;
    sequence?: string;
    associations: AutoTypeAssociation[];
};

export type AutoTypeAssociation = {
    window?: string;
    sequence?: string;
};

export type History = {
    entries: Entry[];
};

export type Group = {
    uuid: string,
    name: string,
    notes?: string,
    icon_id?: number,
    custom_icon_uuid?: string,
    children: Node[],
    times: Times,
    custom_data: CustomData,
    is_expanded: boolean,
    default_autotype_sequence?: string,
    enable_autotype?: string,
    enable_searching?: string,
    last_top_visible_entry?: string,

};
export type Entry = {
    uuid: string,
    fields: Map<string, Value>,
    autotype?: AutoType,
    times: Times,
    custom_data: CustomData,

    icon_id?: number,
    custom_icon_uuid?: string,

    foreground_color?: Color,
    background_color?: Color,

    override_url?: string,
    quality_check?: boolean,

    history?: History,
}

export type CreateAccountRequest = {};

export type UpdateAccountRequest = {};