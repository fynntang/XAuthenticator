import {defaultWindowIcon} from "@tauri-apps/api/app";
import {TrayIcon, type TrayIconEvent, type TrayIconOptions} from "@tauri-apps/api/tray";
import {Menu} from "@tauri-apps/api/menu/menu";
import {showWindow} from "$lib/window";
import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
import {openUrl} from '@tauri-apps/plugin-opener';


export const initialize = () => {
    const trayId = "0199dd1b-2c57-7000-8000-000000000000";
    let tray: TrayIcon | null = null;
    return {
        createTray: async () => {
            tray = await TrayIcon.getById(trayId)
            if (tray) return;
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
                            action: () => showWindow(WebviewWindowLabels.Main)
                        },
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
                        {item: "Separator"},
                        {
                            id: 'settings',
                            text: 'Settings',
                            enabled: true,
                            action: () => showWindow(WebviewWindowLabels.Settings)
                        },
                        {
                            id: 'quit', text: 'Quit App', enabled: true, action: () => {
                                console.log('Quit App');
                            }
                        },
                    ],
                }),
                action: (event: TrayIconEvent) => {
                    if (event.type === 'Click' && event.button === "Left") showWindow(WebviewWindowLabels.Main)
                },
            } as TrayIconOptions);
        },
        removeTray: async () => await TrayIcon.removeById(trayId),
    }
};