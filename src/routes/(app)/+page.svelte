<script lang="ts">
    import {ScrollArea} from "$lib/components/ui/scroll-area";
    import {onMount} from "svelte";
    import {listAccounts} from "$lib/api/api";
    import AccountList from "$lib/components/AccountList.svelte";

    onMount(() => {
        listAccounts()
    })
</script>

<div class="h-[var(--body-height)] w-full">
    <AccountList />
</div>