<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {appState} from "$lib/api/api";
    import {appIsLocked, appStore} from "$lib/stores/stores";
    import type {AppStateResponse} from "$lib/api/types";
    import AppLayout from "$lib/components/layout/app-layout.svelte";
    import {useSidebar} from "$lib/components/ui/sidebar";

    let {children} = $props();
    let timer: number = 0;

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
</script>

<AppLayout>
    {@render children()}
</AppLayout>

<style lang="css">
</style>
