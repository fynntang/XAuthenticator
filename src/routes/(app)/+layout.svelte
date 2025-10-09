<script lang="ts">
    import {Minus, Pin, PinOff, Plus, Search, Settings, X} from '@lucide/svelte';
    import {Button} from "$lib/components/ui/button";
    import {InputGroup, InputGroupAddon, InputGroupInput} from "$lib/components/ui/input-group";
    import {Separator} from "$lib/components/ui/separator";

    import Logo from "$lib/assets/tiny-logo.png";
    import {onMount} from "svelte";
    import {getCurrentWindow, Window} from "@tauri-apps/api/window";

    const appWindow = getCurrentWindow();

    let {children} = $props();
    let isAlwaysOnTop = $state(false);
    let title = $state("XAuthenticator");


    const toggleAlwaysOnTop = () => {
        appWindow.setAlwaysOnTop(!isAlwaysOnTop);
        isAlwaysOnTop = !isAlwaysOnTop;
    }

    const openSettings = async () => {
        const settingWindow = await Window.getByLabel("settings")
        console.log("settingWindow:", settingWindow)

        if (settingWindow) {
            await settingWindow.setFocus();
            return settingWindow;
        }

        // 创建新窗口
        const settingsWindow = new Window('settings', {
            title: 'Settings',
            width: 500,
            height: 400,
            center: true,
            resizable: true,
            parent: Window.getCurrent(),  // 设置父窗口
            decorations: false,  // 无边框窗口
            transparent: true,  // 透明背景
            alwaysOnTop: true  // 保持在最上层
        });
        await settingsWindow.once('tauri://created', () => {
            console.log('设置窗口已创建');
        });
        await settingsWindow.once('tauri://error', (e) => {
            console.error('设置窗口创建失败:', e);
        });

        return settingsWindow;
    }


    onMount(async () => {
        title = await appWindow.title();
    })

</script>

<main>
    <header class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear select-none">
        <div data-tauri-drag-region class="flex w-full items-center gap-1 pl-4 lg:gap-2 lg:pl-6">
            <div data-tauri-drag-region class="flex flex-auto items-center gap-2">
                <div class="flex flex-none items-center gap-2 select-none">
                    <img src={Logo} alt="XAuthenticator Logo" class="size-8">
                    <h1 class="flex-none text-base font-medium">{title}</h1>
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
                <Button variant="ghost" size="icon" class="mr-2" onclick={()=>{
                    openSettings()
                }}>
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
</main>


<style lang="css">
</style>