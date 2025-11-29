<script lang="ts">
    import {type ComponentProps, onMount} from "svelte";
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
        SidebarMenuSub,
        SidebarMenuSubButton,
        SidebarMenuSubItem,
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
        ChevronRightIcon,
        FolderIcon,
        ForwardIcon,
        GalleryVerticalEndIcon,
        SettingsIcon,
        TagsIcon,
        Trash2Icon
    } from "@lucide/svelte"
    import type {Group} from "$lib/api/types";
    import {listGroups, listTags} from "$lib/api/api";

    let {
        ref = $bindable(null),
        ...restProps
    }: ComponentProps<typeof Sidebar> = $props();

    let tags = $state<string[]>([]);
    let groups = $state<Group[]>([]);

    const sidebarState = useSidebar();

    onMount(async () => {
        tags = await listTags()
        groups = await listGroups()
    })

</script>

<Sidebar collapsible="icon" bind:ref {...restProps}>
    <SidebarHeader/>
    <SidebarContent>
        <SidebarGroup>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton tooltipContent={"All Items"}>
                        <GalleryVerticalEndIcon/>
                        <span>All Items</span>
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarGroup>

        <!--
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
        -->
        {#if tags.length > 0}
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
                                        {#each tags as tag,index(index)}
                                            <SidebarMenuItem>
                                                <SidebarMenuButton>
                                                    {#snippet child({props})}
                                                        <button type="button" {...props}><span>{tag}</span></button>
                                                    {/snippet}
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
        {/if}
        <SidebarGroup>
            <SidebarGroupLabel>Groups</SidebarGroupLabel>
            <SidebarMenu>
                {#each groups as group,index(index)}
                    <SidebarMenuItem>
                        <SidebarMenuButton tooltipContent={group.name}>
                            <FolderIcon/>
                            <span>{group.name}</span>
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


                        <SidebarMenuSub>
                            {#each group.children as subGroup,subIndex(subIndex)}
                                {#if subGroup.type === 'Group'}
                                    <SidebarMenuSubItem>
                                        <SidebarMenuSubButton>
                                            {subGroup.Group.name}
                                        </SidebarMenuSubButton>
                                    </SidebarMenuSubItem>
                                {/if}
                            {/each}
                        </SidebarMenuSub>
                    </SidebarMenuItem>
                {/each}
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