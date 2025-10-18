import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";

export const showWindow = async (label: WebviewWindowLabels) => {
    WebviewWindow.getByLabel(label)?.then((window) => {
        if (!window) return;
        window.show()
        window.unminimize()
        window.setFocus()
    });
}
export const hideWindow = async (label: WebviewWindowLabels) => {
    WebviewWindow.getByLabel(label)?.then((window) => {
        if (!window) return;
        window.close()
    });
}