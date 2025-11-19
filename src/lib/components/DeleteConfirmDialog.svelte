<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import {AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle} from "$lib/components/ui/alert-dialog";
    
    export let open = false;
    export let accountName = '';
    
    const dispatch = createEventDispatcher();
    
    function handleConfirm() {
        dispatch('confirm');
    }
</script>

<AlertDialog bind:open>
    <AlertDialogContent>
        <AlertDialogHeader>
            <AlertDialogTitle>Delete Account</AlertDialogTitle>
            <AlertDialogDescription>
                Are you sure you want to delete "<strong>{accountName}</strong>"? This action cannot be undone.
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onclick={handleConfirm}>Delete</AlertDialogAction>
        </AlertDialogFooter>
    </AlertDialogContent>
</AlertDialog>
