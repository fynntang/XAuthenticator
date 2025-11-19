<script lang="ts">
    import AppLockLayout from "./app-lock-layout.svelte";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import Titlebar from "$lib/components/layout/titlebar.svelte";
    import AppSidebar from "$lib/components/layout/app-sidebar.svelte";
    import {SidebarInset} from "$lib/components/ui/sidebar";

    let {children} = $props();
</script>

<main>
    <Titlebar windowLabels={WebviewWindowLabels.Main}/>
    <AppLockLayout>
        <AppSidebar variant="inset"/>
        <SidebarInset>
            <div class="flex flex-1 flex-col">
                <div class="@container/main flex flex-1 flex-col gap-2">
                    <div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                        {@render children?.()}
                    </div>
                </div>
            </div>
        </SidebarInset>
    </AppLockLayout>
</main>
