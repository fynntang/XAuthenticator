<script lang="ts">
    import AppSheet from '$lib/components/app-sheet/app-sheet.svelte';
    import '../app.css';
    import {ModeWatcher} from "mode-watcher";
    import {Plus, Search, TextAlignJustify} from '@lucide/svelte';
    import {Button} from "$lib/components/ui/button";
    import {appSheetStore} from "$lib/stores/app-sheet";
    import {InputGroup, InputGroupAddon, InputGroupInput} from "$lib/components/ui/input-group";


    let {children} = $props();

</script>

<ModeWatcher/>
<AppSheet/>

<main>
    <header class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear select-none">
        <div class="flex w-full items-center gap-1 px-4 lg:gap-2 lg:px-6">
            <Button variant="ghost" size="sm" class="flex-none size-8 mr-2" onclick={appSheetStore.open}>
                <TextAlignJustify/>
            </Button>
            <h1 class="flex-none text-base font-medium">XAuthenticator</h1>
            <InputGroup class="flex ml-auto min-w-3xs max-w-sm">
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
    </header>
    {@render children()}
</main>
