<script lang="ts">


    import {Minus, Pin, PinOff, X} from '@lucide/svelte';
    import {Button} from "$lib/components/ui/button";
    import {Separator} from "$lib/components/ui/separator";
    import {onMount} from "svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import Logo from "$lib/assets/tiny-logo.png";

    const appWindow = getCurrentWindow();
    let {children} = $props();
    let isAlwaysOnTop = $state(false);
    let title = $state("XAuthenticator");

    const toggleAlwaysOnTop = () => {
        appWindow.setAlwaysOnTop(!isAlwaysOnTop);
        isAlwaysOnTop = !isAlwaysOnTop;
    }


    onMount(async () => {
        title = await appWindow.title();
    })

</script>


<main>
    <header class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear select-none">
        <div data-tauri-drag-region class="flex w-full items-center gap-1 pl-4 lg:gap-2 lg:pl-6">
            <div data-tauri-drag-region class="flex flex-auto items-center gap-2 mr-auto">
                <div class="flex flex-none items-center gap-2 select-none">
                    <img src={Logo} alt="XAuthenticator Logo" class="size-8">
                    <h1 class="flex-none text-base font-medium">{title}</h1>
                </div>
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
                <Separator orientation="vertical" class="h-8 mx-1"/>
                <Button variant="ghost" size="icon" class="mr-2" onclick={() => {
                    console.log('minimize');
                    appWindow.minimize();
                    console.log('minimize');
                }}>
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
</main>


