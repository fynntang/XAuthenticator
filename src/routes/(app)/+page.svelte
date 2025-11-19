<script lang="ts">
    import {onMount} from "svelte";
    import {listAccounts} from "$lib/api/api";
    import AccountList from "$lib/components/AccountList.svelte";

    onMount(() => {
        listAccounts()
    })
</script>


<AccountList/>
