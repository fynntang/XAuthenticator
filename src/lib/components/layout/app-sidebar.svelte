<script lang="ts">
    import type {ComponentProps} from "svelte";
    import {
        Sidebar,
        SidebarContent,
        SidebarFooter,
        SidebarGroup,
        SidebarGroupLabel,
        SidebarHeader,
        SidebarMenu,
        SidebarMenuAction,
        SidebarMenuButton,
        SidebarMenuItem,
        SidebarRail,
        useSidebar
    } from "$lib/components/ui/sidebar";
    import {
        DropdownMenu,
        DropdownMenuContent,
        DropdownMenuItem,
        DropdownMenuSeparator,
        DropdownMenuTrigger
    } from "$lib/components/ui/dropdown-menu";

    import {Collapsible, CollapsibleContent, CollapsibleTrigger} from "$lib/components/ui/collapsible";
    import {
        BookmarkIcon,
        ChevronRightIcon,
        FolderIcon,
        ForwardIcon,
        GalleryVerticalEndIcon,
        PaletteIcon,
        SettingsIcon,
        TagsIcon,
        Trash2Icon
    } from "@lucide/svelte"

    let {
        ref = $bindable(null),
        ...restProps
    }: ComponentProps<typeof Sidebar> = $props();


    const sidebarState = useSidebar();

</script>

<Sidebar collapsible="icon" bind:ref {...restProps}>
    <SidebarHeader/>
    <SidebarContent>
        <SidebarGroup>
            <SidebarMenu>
                <SidebarMenuItem>
                    <!-- class="data-[slot=sidebar-menu-button]:!p-1.5" -->
                    <SidebarMenuButton tooltipContent={"All Items"}>
                        <GalleryVerticalEndIcon/>
                        <span>All Items</span>
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarGroup>


        <SidebarGroup>
            <SidebarMenu>
                <Collapsible class="group/collapsible">
                    {#snippet child({props})}
                        <SidebarMenuItem {...props}>
                            <CollapsibleTrigger>
                                {#snippet child({props})}
                                    <SidebarMenuButton {...props} tooltipContent={"Colors"}>
                                        <PaletteIcon/>
                                        <span>Colors</span>
                                        <ChevronRightIcon
                                                class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
                                        />
                                    </SidebarMenuButton>
                                {/snippet}
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenu class="flex-row max-w-full overflow-hidden">
                                    {#each Array.from(["hover:fill-yellow-700 stroke-yellow-700", "hover:fill-green-700 stroke-green-700", "hover:fill-red-700 stroke-red-700", "hover:fill-orange-700 stroke-orange-700", "hover:fill-blue-700 stroke-blue-700", "hover:fill-pink-700 stroke-pink-700"]) as color, index (index)}
                                        <SidebarMenuItem>
                                            <SidebarMenuButton class="p-1.5 gap-2">
                                                <BookmarkIcon class={color}/>
                                            </SidebarMenuButton>
                                        </SidebarMenuItem>
                                    {/each}
                                </SidebarMenu>
                            </CollapsibleContent>
                        </SidebarMenuItem>
                    {/snippet}

                </Collapsible>
            </SidebarMenu>
        </SidebarGroup>

        <SidebarGroup>
            <SidebarMenu>
                <Collapsible class="group/collapsible">
                    {#snippet child({props})}
                        <SidebarMenuItem {...props}>
                            <CollapsibleTrigger>
                                {#snippet child({props})}
                                    <SidebarMenuButton {...props} tooltipContent={"Tags"}>
                                        <TagsIcon/>
                                        <span>Tags</span>
                                        <ChevronRightIcon
                                                class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
                                        />
                                    </SidebarMenuButton>
                                {/snippet}
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            {#snippet child({props})}
                                                <a href="##" {...props}><span>A</span></a>
                                            {/snippet}
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </CollapsibleContent>
                        </SidebarMenuItem>
                    {/snippet}

                </Collapsible>
            </SidebarMenu>
        </SidebarGroup>

        <SidebarGroup>
            <SidebarGroupLabel>Groups</SidebarGroupLabel>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton tooltipContent={"Account"}>
                        <FolderIcon/>
                        <span>Account</span>
                    </SidebarMenuButton>
                    <DropdownMenu>
                        <DropdownMenuTrigger>
                            {#snippet child({props})}
                                <SidebarMenuAction showOnHover {...props}>
                                    <SettingsIcon class="ml-auto"/>
                                </SidebarMenuAction>
                            {/snippet}
                        </DropdownMenuTrigger>
                        <DropdownMenuContent class="w-48 rounded-lg"
                                             side={sidebarState.isMobile ? "bottom" : "right"}
                                             align={sidebarState.isMobile ? "end" : "start"}>
                            <DropdownMenuItem>
                                <FolderIcon class="text-muted-foreground"/>
                                <span>View Project</span>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <ForwardIcon class="text-muted-foreground"/>
                                <span>Share Project</span>
                            </DropdownMenuItem>
                            <DropdownMenuSeparator/>
                            <DropdownMenuItem>
                                <Trash2Icon class="text-muted-foreground"/>
                                <span>Delete Project</span>
                            </DropdownMenuItem>

                        </DropdownMenuContent>

                    </DropdownMenu>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarGroup>
    </SidebarContent>
    <SidebarFooter>
        <SidebarGroup>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton tooltipContent={"Recently deleted"}>
                        <Trash2Icon/>
                        Recently deleted
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarGroup>
    </SidebarFooter>
    <SidebarRail/>
</Sidebar>