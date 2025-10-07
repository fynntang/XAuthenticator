<script lang="ts">
    import {
        Sheet,
        SheetContent,
        SheetDescription,
        SheetFooter,
        SheetHeader,
        SheetTitle
    } from "$lib/components/ui/sheet";
    import {Button} from "$lib/components/ui/button";
    import {appSheetStore} from "$lib/stores/app-sheet";
</script>


<Sheet open={$appSheetStore} onOpenChange={appSheetStore.set} onOpenChangeComplete={appSheetStore.set}>
    <SheetContent side="left">
        <SheetHeader>
            <SheetTitle>Sheet title</SheetTitle>
            <SheetDescription>
                Lorem ipsum dolor sit amet consectetur adipisicing elit.
            </SheetDescription>
        </SheetHeader>
        <SheetFooter>
            <Button type="submit">Save</Button>
        </SheetFooter>
    </SheetContent>
</Sheet>
