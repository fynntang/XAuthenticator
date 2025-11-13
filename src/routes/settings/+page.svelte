<script lang="ts">
    import {Separator} from "$lib/components/ui/separator";
    import {_ as t} from 'svelte-i18n';
</script>


<main class="flex flex-row w-full h-full items-center gap-1 p-4 lg:gap-2 lg:p-6 border-r transition-[width,height] ease-linear">
    <div class="flex flex-1/3">
        <div class="flex flex-col">
            aaaa
        </div>
    </div>
    <Separator orientation="vertical"/>
    <div class="flex flex-2/3">
        <h1>{$t('settings.title')}</h1>
    </div>
</main>

