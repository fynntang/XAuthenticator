<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {appState} from "$lib/api/api";
    import {appIsLocked, appStore} from "$lib/stores/stores";
    import type {AppStateResponse} from "$lib/api/types";
    import AppLayout from "$lib/components/layout/app-layout.svelte";

    const appWindow = getCurrentWindow();

    let {children} = $props();
    let isAlwaysOnTop = $state(false);
    let timer: number = 0;


    const toggleAlwaysOnTop = () => {
        appWindow.setAlwaysOnTop(!isAlwaysOnTop);
        isAlwaysOnTop = !isAlwaysOnTop;
    }

    const appStateChange = (v: AppStateResponse) => {
        appStore.set(v)
        appIsLocked.update(v => $appStore?.isLocked ? true : v)
    }


    onMount(() => {
        appState().then(appStateChange)
        timer = setInterval(async () => {
            const data = await appState();
            appStateChange(data)
        }, 3000);
    })
    onDestroy(() => {
        if (timer > 0) clearInterval(timer);
    })

    $inspect($appStore, $appIsLocked)
</script>


<AppLayout>
    {@render children()}
</AppLayout>


<style lang="css">
</style>
