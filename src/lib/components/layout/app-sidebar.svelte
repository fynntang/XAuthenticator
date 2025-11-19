<script lang="ts">
    import type {ComponentProps} from "svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import {
        SidebarContent,
        SidebarFooter,
        SidebarHeader,
        SidebarMenu,
        SidebarMenuButton,
        SidebarMenuItem,
        SidebarRail
    } from "$lib/components/ui/sidebar/index.js";


    let {
        ref = $bindable(null),
        ...restProps
    }: ComponentProps<typeof Sidebar.Root> = $props();
    let rail = null;

    const resizeSidebar = () => {

    }

</script>

<Sidebar.Root bind:ref {...restProps}>
    <SidebarHeader>
        <SidebarMenu>
            <SidebarMenuItem>
                <SidebarMenuButton class="data-[slot=sidebar-menu-button]:!p-1.5">
                    {#snippet child({props})}
                        <a href="##" {...props}>
                            <span class="text-base font-semibold">Acme Inc.</span>
                        </a>
                    {/snippet}
                </SidebarMenuButton>
            </SidebarMenuItem>
        </SidebarMenu>
    </SidebarHeader>
    <SidebarContent>

    </SidebarContent>
    <SidebarFooter>

    </SidebarFooter>
    <SidebarRail bind:this={rail}/>
</Sidebar.Root>