<script lang="ts">
    import {Minus, Pin, PinOff, Plus, Search, Settings, X} from '@lucide/svelte';
    import {Button} from "$lib/components/ui/button";
    import {InputGroup, InputGroupAddon, InputGroupInput} from "$lib/components/ui/input-group";
    import {Separator} from "$lib/components/ui/separator";
    import {onDestroy, onMount} from "svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import logo from "$lib/assets/logo.png";
    import {showWindow} from "$lib/window";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import {appState, healthCheck} from "$lib/api/api";
    import AppLockLayout from '$lib/components/layout/app-lock-layout.svelte';
    import {appStore} from "$lib/stores/stores";
    import type {AppStateResponse} from "$lib/api/types";

    const appWindow = getCurrentWindow();

    let {children} = $props();
    let isAlwaysOnTop = $state(false);
    let isLocked = $state(false);
    let timer: number = 0;


    const toggleAlwaysOnTop = () => {
        appWindow.setAlwaysOnTop(!isAlwaysOnTop);
        isAlwaysOnTop = !isAlwaysOnTop;
    }

    const appStateChange = (v: AppStateResponse) => {
        appStore.set(v)
        if ($appStore?.config.builder.settings.autoLock) isLocked = $appStore?.isLocked ?? false;
    }

    onMount(() => {
        appState().then(appStateChange)
        timer = setInterval(async () => {
            try {
                await healthCheck();
            } catch (e) {
                // 如果健康检查触发锁定，重新拉取状态
            }
            const data = await appState();
            appStateChange(data)
        }, 3000);
    })
    onDestroy(() => {
        if (timer > 0) clearInterval(timer);
    })

    $inspect($appStore, isLocked)
</script>

<main>
    <AppLockLayout isLocked>
        <header class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear select-none">
            <div data-tauri-drag-region class="flex w-full items-center gap-1 pl-4 lg:gap-2 lg:pl-6">
                <div data-tauri-drag-region class="flex flex-auto items-center gap-2">
                    <div data-tauri-drag-region class="flex flex-none items-center gap-2 select-none">
                        <div data-tauri-drag-region class="flex-none size-8"
                             style:background-image="url({logo})"
                             style:background-size="cover"
                             style:background-position="center"
                        ></div>
                        <h1 data-tauri-drag-region class="flex-none text-xl font-medium">{__NAME__}</h1>
                    </div>
                    <InputGroup class="flex mx-auto min-w-3xs max-w-sm">
                        <InputGroupInput placeholder="Search..."/>
                        <InputGroupAddon>
                            <Search/>
                        </InputGroupAddon>
                    </InputGroup>
                    <Button variant="ghost" size="sm" class="dark:text-foreground hidden sm:flex">
                        Add New
                        <Plus/>
                    </Button>
                </div>
                <div class="flex flex-none ml-auto items-center">
                    <Button variant="ghost" size="icon"
                            class="mr-2 {isAlwaysOnTop?'bg-accent text-accent-foreground dark:bg-accent/50':''}"
                            onclick={toggleAlwaysOnTop}>
                        {#if isAlwaysOnTop}
                            <PinOff/>
                        {:else}
                            <Pin/>
                        {/if}
                    </Button>
                    <Button variant="ghost" size="icon" class="mr-2"
                            onclick={()=>showWindow(WebviewWindowLabels.Settings)}>
                        <Settings/>
                    </Button>
                    <Separator orientation="vertical" class="h-8 mx-1"/>
                    <Button variant="ghost" size="icon" class="mr-2" onclick={() => appWindow.minimize()}>
                        <Minus/>
                    </Button>
                    <Button variant="ghost" size="icon-lg" onclick={() => appWindow.close()}
                            class="rounded-none hover:!bg-red-500 hover:!text-white">
                        <X/>
                    </Button>
                </div>
            </div>
        </header>
        {@render children()}
    </AppLockLayout>
</main>


<style lang="css">
</style>
