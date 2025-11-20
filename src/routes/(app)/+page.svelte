<script lang="ts">
    import {SIDEBAR_WIDTH} from "$lib/components/ui/sidebar/constants";
    import {ResizableHandle, ResizablePane, ResizablePaneGroup} from "$lib/components/ui/resizable";
    import {Item, ItemContent, ItemDescription, ItemGroup, ItemMedia, ItemTitle} from "$lib/components/ui/item";
    import {Avatar, AvatarFallback, AvatarImage} from "$lib/components/ui/avatar";
    import {ScrollArea} from "$lib/components/ui/scroll-area";

    const sidebarWidth = Number(SIDEBAR_WIDTH.replace("rem", "")) * 16;

</script>


<ResizablePaneGroup direction="horizontal">
    <ResizablePane minSize={25} defaultSize={30} maxSize={35} class="flex flex-col pl-4">
        <ScrollArea class="h-full w-full">
            <ItemGroup class="pr-4">
                {#each Array.from({length: 500}) as _,index(index)}
                    <Item>
                        {#snippet child({props})}
                            <a href="#/" {...props}>
                                <ItemMedia>
                                    <Avatar>
                                        <AvatarImage
                                                src={"https://ui-avatars.com/api/?name=%E6%B7%98%E5%AE%9D&format=svg&bold=true"}
                                                class="grayscale"/>
                                        <AvatarFallback>Name</AvatarFallback>
                                    </Avatar>
                                </ItemMedia>
                                <ItemContent class="gap-1">
                                    <ItemTitle>Name</ItemTitle>
                                    <ItemDescription>email</ItemDescription>
                                </ItemContent>
                            </a>
                        {/snippet}
                    </Item>
                {/each}
            </ItemGroup>
        </ScrollArea>
    </ResizablePane>
    <ResizableHandle withHandle/>
    <ResizablePane minSize={65} defaultSize={70} maxSize={75} class="flex flex-col px-4">
        <ScrollArea class="h-full w-full">
            aaa
        </ScrollArea>
    </ResizablePane>
</ResizablePaneGroup>