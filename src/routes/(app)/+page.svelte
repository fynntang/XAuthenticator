<script lang="ts">
    import {ScrollArea} from "$lib/components/ui/scroll-area";
    import AccountList from "$lib/components/AccountList.svelte";
</script>

<ScrollArea class="h-[var(--body-height)] w-full">
    <div class="relative flex flex-col overflow-hidden p-6">
        <AccountList />
    </div>
</ScrollArea>
