<script lang="ts">
    import Logo from "$lib/assets/tiny-logo.png";
    import {Button} from "$lib/components/ui/button";
    import {X} from "@lucide/svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";

    const appWindow = getCurrentWindow();

    let {children} = $props();
    let isAlwaysOnTop = $state(false);
    let title = $state("Settings");

    const toggleAlwaysOnTop = () => {
        appWindow.setAlwaysOnTop(!isAlwaysOnTop);
        isAlwaysOnTop = !isAlwaysOnTop;
    }

</script>

<header class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear select-none">
    <div data-tauri-drag-region class="flex w-full items-center gap-1 pl-4 lg:gap-2 lg:pl-6">
        <div data-tauri-drag-region class="flex flex-auto items-center gap-2">
            <div class="flex flex-none items-center gap-2 select-none">
                <img src={Logo} alt="XAuthenticator Logo" class="size-8">
                <h1 class="flex-none text-base font-medium">{title}</h1>
            </div>
        </div>
        <div class="flex flex-none ml-auto items-center">
            <Button variant="ghost" size="icon-lg" onclick={() => appWindow.close()}
                    class="rounded-none hover:!bg-red-500 hover:!text-white">
                <X/>
            </Button>
        </div>
    </div>
</header>

{@render children()}
