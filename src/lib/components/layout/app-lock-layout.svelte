<script lang="ts">

    import type {WithElementRef} from "$lib/utils";
    import type {HTMLAttributes} from "svelte/elements";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import Titlebar from "$lib/components/layout/titlebar.svelte";

    let {
        isLocked = false,
        children
    }: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
        isLocked: boolean | undefined,
    } = $props();
</script>

<Titlebar isLocked windowLabels={ WebviewWindowLabels.Main }/>
{#if isLocked}
    <h1>Loading...</h1>
{:else }
    {@render children?.()}
{/if}


