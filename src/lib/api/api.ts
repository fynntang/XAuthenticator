import {invoke, type InvokeArgs, type InvokeOptions,} from "@tauri-apps/api/core";
import type {Account, APIError, AppDefault, AppStateResponse, CreateAccountRequest, InitRequest, UpdateAccountRequest,} from "$lib/api/types";


export const appDefault = async () => await apiInvoke<AppDefault>("app_default");
export const initApp = async (request: InitRequest) => await apiInvoke<void>("init_app", {request});
export const launchApp = async () => await apiInvoke<void>("launch_app");
export const appState = async () => await apiInvoke<AppStateResponse>("app_state");
export const quitApp = async () => await apiInvoke<void>("quit_app");
export const lockApp = async () => await apiInvoke<void>("lock");
export const unlockAppWithPassword = async (password: string) => await apiInvoke<void>("unlock_with_password", {password});

export const listAccounts = async () => await apiInvoke<Account[]>("list_accounts");
export const createAccount = async (request: CreateAccountRequest) => await apiInvoke<Account>("create_account", {request});
export const updateAccount = async (request: UpdateAccountRequest) => await apiInvoke<Account>("update_account", {request});
export const deleteAccount = async (accountId: string) => await apiInvoke<void>("delete_account", {accountId});
export const getCode = async (accountId: string) => await apiInvoke<string>("get_code", {accountId});

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
