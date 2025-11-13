import {TrayIcon, type TrayIconEvent, type TrayIconOptions} from "@tauri-apps/api/tray";
import {defaultWindowIcon} from "@tauri-apps/api/app";
import {Menu} from "@tauri-apps/api/menu/menu";
import {showWindow} from "$lib/window";
import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
import {openUrl} from "@tauri-apps/plugin-opener";
import {lockApp, quitApp} from "$lib/api/api";
import {appIsLocked} from "$lib/stores/stores";
import {Submenu} from "@tauri-apps/api/menu/submenu";
import {MenuItem} from "@tauri-apps/api/menu/menuItem";
import {PredefinedMenuItem} from "@tauri-apps/api/menu/predefinedMenuItem";
import {CheckMenuItem} from "@tauri-apps/api/menu/checkMenuItem";
import {IconMenuItem} from "@tauri-apps/api/menu/iconMenuItem";
import {get} from "svelte/store";

const trayId = "00000000-0000-0000-0000-000000000000";
let lockMenuItem: Submenu | MenuItem | PredefinedMenuItem | CheckMenuItem | IconMenuItem | null = null;
export const createTray = async () => {
    const trayIcon = await TrayIcon.getById(trayId)
    if (trayIcon) {
        await TrayIcon.removeById(trayId)
    }
    const menu = await Menu.new({
        items: [
            {
                id: 'open',
                text: 'Open App',
                enabled: true,
                accelerator: "CmdOrCtrl+O",
                action: () => showWindow(WebviewWindowLabels.Main)
            },
            {item: "Separator"},
            {
                id: 'official_website', text: 'Official Website', enabled: true, action: () => {
                    console.log('Official Website', __REPOSITORY__);
                    openUrl(__REPOSITORY__)
                    console.log("Opened")
                },
            },
            {
                id: 'about', text: 'About', enabled: true, action: () => {
                    console.log('About');
                },
            },
            {
                id: 'check-for-update', text: 'Check for Updates', enabled: true, action: () => {
                    console.log('Check for Updates');
                },
            },
            {item: "Separator"},
            {
                id: 'lock', text: 'Lock', enabled: true, action: async (id) => {
                    lockMenuItem = await menu.get(id)
                    await toggleLock();
                }
            },
            {id: 'settings', text: 'Settings', enabled: true, action: () => showWindow(WebviewWindowLabels.Settings)},
            {id: 'quit', text: 'Quit App', enabled: true, accelerator: "CmdOrCtrl+Q", action: () => quitApp()},
        ],
    })

    const newTrayIcon = await TrayIcon.new({
        id: trayId,
        icon: await defaultWindowIcon(),
        tooltip: __NAME__,
        showMenuOnLeftClick: false,
        menu,
        action: (event: TrayIconEvent) => {
            if (event.type === 'Click' && event.button === "Left") showWindow(WebviewWindowLabels.Main)
        },
    } as TrayIconOptions);

    lockMenuItem = await menu.get("lock");

    appIsLocked.subscribe(async (isLocked: boolean) => {
        console.log("appIsLocked subscribe", isLocked);
        if (lockMenuItem) {
            await lockMenuItem.setText(isLocked ? "Unlock" : "Lock");
        }
    })

    return newTrayIcon;
}

export const removeTray = async () => {
    const trayIcon = await TrayIcon.getById(trayId)
    if (trayIcon) {
        await TrayIcon.removeById(trayId)
    }
}

export const getTray = async () => {
    return await TrayIcon.getById(trayId)
}

async function toggleLock() {
    const locked = get(appIsLocked);
    if (!locked) {
        await lockApp();
    } else {
        console.log("await Unlock", locked);
    }
}
