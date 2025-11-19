import {invoke, type InvokeArgs, type InvokeOptions,} from "@tauri-apps/api/core";
import type {Account, APIError, AppDefault, AppStateResponse, InitRequest,} from "$lib/api/types";


export const appDefault = async () => await apiInvoke<AppDefault>("app_default");
export const initApp = async (request: InitRequest) => await apiInvoke<void>("init_app", {request});
export const launchApp = async () => await apiInvoke<void>("launch_app");
export const appState = async () => await apiInvoke<AppStateResponse>("app_state");
export const quitApp = async () => await apiInvoke<void>("quit_app");
export const lockApp = async () => await apiInvoke<void>("lock");
export const unlockAppWithPassword = async (password: string) => await apiInvoke<void>("unlock_with_password", {password});

export const listAccounts = async () => await apiInvoke<Account[]>("list_accounts");

async function apiInvoke<T>(
    cmd: string,
    args?: InvokeArgs,
    options?: InvokeOptions
) {
    const resp = await invoke<T | APIError>(cmd, args, options);
    if (!resp) {
        return resp as T;
    }
    if ("code" in (resp as APIError)) {
        return Promise.reject(resp as APIError);
    }
    return resp as T;
}
