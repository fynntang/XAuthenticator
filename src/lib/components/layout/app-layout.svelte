<script lang="ts">
    import AppLockLayout from "./app-lock-layout.svelte";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import Titlebar from "$lib/components/layout/titlebar.svelte";
    import AppSidebar from "$lib/components/layout/app-sidebar.svelte";
    import {SidebarInset, SidebarProvider} from "$lib/components/ui/sidebar";
    import {SIDEBAR_WIDTH, SIDEBAR_WIDTH_ICON} from "$lib/components/ui/sidebar/constants";

    let {children} = $props();
</script>

<main>
    <Titlebar windowLabels={WebviewWindowLabels.Main}/>
    <AppLockLayout>
        <SidebarProvider
                class="flex flex-col min-h-[calc(100svh-var(--header-height))] h-[calc(100svh-var(--header-height))]!"
                style="--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON};">
            <div class="flex flex-1">
                <AppSidebar variant="inset" class="top-(--header-height) h-[calc(100svh-var(--header-height))]!"/>
                <SidebarInset>
                    <div class="flex flex-1 flex-col">
                        <div class="@container/main flex flex-1 flex-col gap-2">
                            <div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                                {@render children?.()}
                            </div>
                        </div>
                    </div>
                </SidebarInset>
            </div>
        </SidebarProvider>
    </AppLockLayout>
</main>
