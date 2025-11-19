<script lang="ts">
    import AppLockLayout from "./app-lock-layout.svelte";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import Titlebar from "$lib/components/layout/titlebar.svelte";

    let {children} = $props();
</script>

<main>
    <Titlebar windowLabels={WebviewWindowLabels.Main}/>
    <AppLockLayout>
        {@render children?.()}
    </AppLockLayout>
</main>
