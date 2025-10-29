import {invoke, type InvokeArgs, type InvokeOptions} from "@tauri-apps/api/core";
import type {Account, APIError, AppStateResponse, PageParam, Response} from "$lib/api/types";

export const launchApp = async () => await apiInvoke<void>("launch_app")
export const appState = async () => await apiInvoke<AppStateResponse>("app_state")
export const lockApp = async () => await apiInvoke<void>("lock")
export const unlockAppWithPassword = async (password: string) => await apiInvoke<void>("unlock_with_password", {password})
export const listAccounts = async (current: number = 0, size: number = 16) => await apiInvoke<Response<Account[]>>("list_accounts", {
    current,
    size
} as PageParam)

// 初始化 MasterKey（后端需实现 tauri 命令：initialize_master_key）
export const initializeMasterKey = async (password: string) => await apiInvoke<void>("initialize_master_key", { password })


async function apiInvoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions) {
    const resp = await invoke<T | APIError>(cmd, args, options);
    if ('code' in (resp as APIError)) {
        return Promise.reject(resp as APIError);
    }
    return resp as T;
}
