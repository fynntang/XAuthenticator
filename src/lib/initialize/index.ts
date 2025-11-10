import {defaultWindowIcon} from "@tauri-apps/api/app";
import {TrayIcon, type TrayIconEvent, type TrayIconOptions} from "@tauri-apps/api/tray";
import {Menu} from "@tauri-apps/api/menu/menu";
import {showWindow} from "$lib/window";
import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
import {openUrl} from '@tauri-apps/plugin-opener';
import {lockApp, quitApp} from "$lib/api/api";


export const initialize = () => {
    const trayId = "00000000-0000-0000-0000-000000000000";
    let tray: TrayIcon | null = null;
    return {
        createTray: async () => {
            tray = await TrayIcon.getById(trayId)
            if (tray) return tray;
            tray = await TrayIcon.new({
                id: trayId,
                icon: await defaultWindowIcon(),
                tooltip: "XAuthenticator",
                showMenuOnLeftClick: false,
                menu: await Menu.new({
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
                            id: 'official_website',
                            text: 'Official Website',
                            enabled: true,
                            action: () => {
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
                            id: 'lock',
                            text: 'Lock',
                            enabled: true,
                            action: () => lockApp()
                        },
                        {
                            id: 'settings',
                            text: 'Settings',
                            enabled: true,
                            action: () => showWindow(WebviewWindowLabels.Settings)
                        },
                        {
                            id: 'quit',
                            text: 'Quit App',
                            enabled: true,
                            accelerator: "CmdOrCtrl+Q",
                            action: () => quitApp()
                        },
                    ],
                }),
                action: (event: TrayIconEvent) => {
                    if (event.type === 'Click' && event.button === "Left") showWindow(WebviewWindowLabels.Main)
                },
            } as TrayIconOptions);
            return tray;
        },
        removeTray: async () => await TrayIcon.removeById(trayId),
        getTray: () => tray,
    }
};