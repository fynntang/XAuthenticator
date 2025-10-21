import {invoke} from "@tauri-apps/api/core";
import type {AccountResponse, AppStateResponse, PageParam, Response} from "$lib/api/types";

export const initApp = async () => await invoke("init_app")
export const appState = async () => await invoke<AppStateResponse>("app_state")
export const lockApp = async () => await invoke<void>("lock")
export const unlockAppWithPassword = async (password: string) => await invoke<void>("unlock_with_password", {password})
export const listAccounts = async (current: number = 0, size: number = 16) =>
    await invoke<Response<AccountResponse[]>>("list_accounts", {
        current,
        size,
    } as PageParam)