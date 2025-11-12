import {invoke, type InvokeArgs, type InvokeOptions,} from "@tauri-apps/api/core";
import type {Account, APIError, AppDefault, AppStateResponse, InitRequest, PageParam, Response,} from "$lib/api/types";


export const appDefault = async () => await apiInvoke<AppDefault>("app_default");
export const initApp = async (request: InitRequest) => await apiInvoke<Response<boolean>>("init_app", {request});
export const launchApp = async () => await apiInvoke<void>("launch_app");
export const appState = async () => await apiInvoke<AppStateResponse>("app_state");
export const quitApp = async () => await apiInvoke<void>("quit_app");
export const lockApp = async () => await apiInvoke<void>("lock");
export const unlockAppWithPassword = async (password: string) => await apiInvoke<void>("unlock_with_password", {password});
export const healthCheck = async () => await apiInvoke<void>("health_check");

export const listAccounts = async (current: number = 0, size: number = 16) => await apiInvoke<Response<Account[]>>("list_accounts", {
    current,
    size,
} as PageParam);

async function apiInvoke<T>(
    cmd: string,
    args?: InvokeArgs,
    options?: InvokeOptions
): Promise<T | void> {
    const resp = await invoke<T | APIError>(cmd, args, options);
    if (!resp) {
        return resp as T;
    }
    if ("code" in (resp as APIError)) {
        return Promise.reject(resp as APIError);
    }
    return resp as T;
}
