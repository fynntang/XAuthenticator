import {
  invoke,
  type InvokeArgs,
  type InvokeOptions,
} from "@tauri-apps/api/core";
import type {
  Account,
  APIError,
  AppStateResponse,
  PageParam,
  Response,
  UpdateAccountRequest,
} from "$lib/api/types";

export const initApp = async (password: string) =>
  await apiInvoke<void>("init_app", { password });
export const launchApp = async () => await apiInvoke<void>("launch_app");
export const appState = async () =>
  await apiInvoke<AppStateResponse>("app_state");
export const quitApp = async () => await apiInvoke<void>("quit_app");
export const lockApp = async () => await apiInvoke<void>("lock");
export const unlockAppWithPassword = async (password: string) =>
  await apiInvoke<void>("unlock_with_password", { password });
export const healthCheck = async () => await apiInvoke<void>("health_check");
export const authCapabilities = async () =>
  await apiInvoke<{
    biometricSupported: boolean;
    pinSupported: boolean;
    methods: string[];
  }>("auth_capabilities");

export const listAccounts = async (current: number = 0, size: number = 16) =>
  await apiInvoke<Response<Account[]>>("list_accounts", {
    current,
    size,
  } as PageParam);

export const addAccount = async (authUrl: string) =>
  await apiInvoke<void>("add_account", { authUrl });

export const updateAccount = async (request: UpdateAccountRequest) =>
  await apiInvoke<void>("update_account", { request });

export const getAccountById = async (accountId: string) =>
  await apiInvoke<Account>("get_account_by_id", { accountId });

export const removeAccount = async (accountId: string) =>
  await apiInvoke<void>("remove_account", { accountId });

async function apiInvoke<T>(
  cmd: string,
  args?: InvokeArgs,
  options?: InvokeOptions
) {
  const resp = await invoke<T | APIError>(cmd, args, options);
  if ("code" in (resp as APIError)) {
    return Promise.reject(resp as APIError);
  }
  return resp as T;
}
