<script lang="ts">
    import logo from "$lib/assets/logo.png";
    import { showWindow } from "$lib/window";
    import { WebviewWindowLabels } from "$lib/constants/webview-window-labels";
    import {
        InputGroup,
        InputGroupAddon,
        InputGroupInput,
    } from "$lib/components/ui/input-group";
    import { Button } from "$lib/components/ui/button";
    import {
        Minus,
        Pin,
        PinOff,
        Plus,
        Search,
        Settings,
        X,
    } from "@lucide/svelte";
    import { Separator } from "$lib/components/ui/separator";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import type { WithElementRef } from "$lib/utils";
    import type { HTMLAttributes } from "svelte/elements";
    import { appStore, accountDialogState } from "$lib/stores/stores";

    const appWindow = getCurrentWindow();

    let {
        windowLabels,
        children,
    }: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
        windowLabels: WebviewWindowLabels | undefined;
    } = $props();
    let isAlwaysOnTop = $state(false);

    const toggleAlwaysOnTop = () => {
        appWindow.setAlwaysOnTop(!isAlwaysOnTop);
        isAlwaysOnTop = !isAlwaysOnTop;
    };
</script>

<header
    class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear select-none"
>
    <div
        data-tauri-drag-region
        class="flex w-full items-center gap-1 pl-4 lg:gap-2 lg:pl-6"
    >
        <div data-tauri-drag-region class="flex flex-auto items-center gap-2">
            <div
                data-tauri-drag-region
                class="flex flex-none items-center gap-2 select-none"
            >
                <div
                    data-tauri-drag-region
                    class="flex-none size-8"
                    style:background-image="url({logo})"
                    style:background-size="cover"
                    style:background-position="center"
                ></div>
                <h1
                    data-tauri-drag-region
                    class="flex-none text-xl font-medium"
                >
                    {__NAME__}
                </h1>
            </div>
            {#if windowLabels === WebviewWindowLabels.Main && !$appStore?.isLocked}
                <InputGroup class="flex mx-auto min-w-3xs max-w-sm">
                    <InputGroupInput placeholder="Search..." />
                    <InputGroupAddon>
                        <Search />
                    </InputGroupAddon>
                </InputGroup>
                <Button
                    variant="ghost"
                    size="sm"
                    class="dark:text-foreground hidden sm:flex"
                    onclick={() =>
                        accountDialogState.set({ open: true, mode: "create" })}
                >
                    Add New
                    <Plus />
                </Button>
            {/if}
        </div>
        <div class="flex flex-none ml-auto items-center">
            {#if windowLabels === WebviewWindowLabels.Main && !$appStore?.isLocked}
                <Button
                    variant="ghost"
                    size="icon"
                    class="mr-2 {isAlwaysOnTop
                        ? 'bg-accent text-accent-foreground dark:bg-accent/50'
                        : ''}"
                    onclick={toggleAlwaysOnTop}
                >
                    {#if isAlwaysOnTop}
                        <PinOff />
                    {:else}
                        <Pin />
                    {/if}
                </Button>
            {/if}
            {#if children}
                {@render children()}
            {/if}
            {#if windowLabels === WebviewWindowLabels.Main && !$appStore?.isLocked}
                <Button
                    variant="ghost"
                    size="icon"
                    class="mr-2"
                    onclick={() => showWindow(WebviewWindowLabels.Settings)}
                >
                    <Settings />
                </Button>
                <Separator orientation="vertical" class="h-8 mx-1" />
                <Button
                    variant="ghost"
                    size="icon"
                    class="mr-2"
                    onclick={() => appWindow.minimize()}
                >
                    <Minus />
                </Button>
            {/if}
            <Button
                variant="ghost"
                size="icon-lg"
                onclick={() => appWindow.close()}
                class="rounded-none hover:!bg-red-500 hover:!text-white"
            >
                <X />
            </Button>
        </div>
    </div>
</header>
