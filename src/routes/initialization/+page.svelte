<script lang="ts">
    import type {PageProps} from './$types'
    import {InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput} from '$lib/components/ui/input-group'
    import {Eye, EyeOff, FolderOpen} from '@lucide/svelte'
    import {randomLaunchImage, wait} from '$lib/utils'
    import {appState, initApp} from '$lib/api/api'
    import {getCurrentWindow} from '@tauri-apps/api/window'
    import {save} from '@tauri-apps/plugin-dialog'
    import {Field, FieldGroup, FieldLabel} from "$lib/components/ui/field";
    import {Button} from "$lib/components/ui/button";
    import {Slider} from "$lib/components/ui/slider";
    import {showWindow} from "$lib/window";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";


    type StrengthLevel = 'weak' | 'fair' | 'good' | 'strong' | 'excellent'
    type Strength = { score: number; label: string; level: StrengthLevel }
    type InitForm = {
        kdbxPath: string;
        password: string;
        confirm: string;
        showPassword: boolean;
        showConfirm: boolean;
        advanced: boolean;
        length: number
    }

    let {data}: PageProps = $props()

    let form: InitForm = $state({
        kdbxPath: data.appDefault.kdbxPath,
        password: '',
        confirm: '',
        showPassword: false,
        showConfirm: false,
        advanced: false,
        length: 16
    })
    let strength = $state<Strength>({score: 0, label: '弱', level: 'weak'})
    let canSubmit = $state(false)
    let loading = $state(false)
    let error = $state('')
    let success = $state('')
    let sliderVal = $state(form.length)

    const labels: Record<StrengthLevel, string> = {
        weak: '弱',
        fair: '一般',
        good: '良好',
        strong: '强',
        excellent: '极强'
    }

    const calcStrength = (pwd: string): Strength => {
        const len = pwd.length
        const hasUpper = /[A-Z]/.test(pwd)
        const hasLower = /[a-z]/.test(pwd)
        const hasDigit = /\d/.test(pwd)
        const hasSpecial = /[^\w\s]/.test(pwd)
        let score = 0
        score += Math.min(40, len * 2)
        score += hasUpper ? 15 : 0
        score += hasLower ? 15 : 0
        score += hasDigit ? 15 : 0
        score += hasSpecial ? 15 : 0
        score = Math.min(100, score)
        const level: StrengthLevel = score < 25 ? 'weak' : score < 50 ? 'fair' : score < 70 ? 'good' : score < 85 ? 'strong' : 'excellent'
        return {score, label: labels[level], level}
    }

    const validate = (): boolean => {
        const p = form.password
        const okLen = p.length >= 12
        const okUpper = /[A-Z]/.test(p)
        const okLower = /[a-z]/.test(p)
        const okDigit = /\d/.test(p)
        const okSpecial = /[^\w\s]/.test(p)
        const match = p && form.confirm && p === form.confirm
        return okLen && okUpper && okLower && okDigit && okSpecial && !!match
    }

    $effect(() => {
        strength = calcStrength(form.password)
        canSubmit = validate()
        form.length = sliderVal
    })

    const pickPath = async () => {
        const result = await save({
            defaultPath: form.kdbxPath,
            filters: [{name: 'KeePass Database', extensions: ['kdbx']}]
        })
        if (result) form.kdbxPath = result
    }

    const genPassword = (len: number) => {
        const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{};:,./?'
        let out = ''
        for (let i = 0; i < len; i++) out += chars[Math.floor(Math.random() * chars.length)]
        form.password = out
        form.confirm = out
    }

    const onSubmit = async () => {
        if (!canSubmit || loading) return
        loading = true
        error = ''
        success = ''
        try {
            await initApp({
                kdbxPath: form.kdbxPath,
                password: form.password,
            })
            await appState()
            success = '初始化成功'
            await wait(1500)
            await showWindow(WebviewWindowLabels.Main)
            await getCurrentWindow().hide()
        } catch (e: any) {
            error = e?.reason ?? e?.message ?? '初始化失败'
        } finally {
            loading = false
        }
    }
</script>

<div data-tauri-drag-region class="grid min-h-svh lg:grid-cols-2">
    <div data-tauri-drag-region class="flex flex-col gap-4 p-6 md:p-10">

        <div class="flex flex-1 items-center justify-center">
            <div class="w-full max-w-xl">
                <div class="flex flex-col gap-6">
                    <FieldGroup>
                        <div class="flex flex-col items-center gap-1 text-center">
                            <h1 class="text-2xl font-bold">创建管理密码</h1>
                            <p class="text-muted-foreground text-balance text-sm">

                            </p>
                        </div>
                        <Field>
                            <FieldLabel for="kdbxPath">数据库</FieldLabel>
                            <InputGroup>
                                <InputGroupInput name="kdbxPath" readonly placeholder="路径"
                                                 bind:value={form.kdbxPath}/>
                                <InputGroupAddon align="inline-end">
                                    <InputGroupButton size="icon-xs" onclick={pickPath}
                                                      title="选择路径" aria-label="选择路径">
                                        <FolderOpen/>
                                    </InputGroupButton>
                                </InputGroupAddon>
                            </InputGroup>
                        </Field>
                        <Field>
                            <FieldLabel for="new-password">管理密码</FieldLabel>
                            <InputGroup>
                                <InputGroupInput name="new-password" type={form.showPassword ? 'text' : 'password'}
                                                 bind:value={form.password} placeholder="密码"
                                                 autocomplete="new-password" autofocus/>
                                <InputGroupAddon align="inline-end">
                                    <InputGroupButton size="icon-xs"
                                                      onclick={() => (form.showPassword = !form.showPassword)}
                                                      title={form.showPassword ? '隐藏密码' : '显示密码'}
                                                      aria-label={form.showPassword ? '隐藏密码' : '显示密码'}>
                                        {#if form.showPassword}
                                            <EyeOff/>
                                        {:else}
                                            <Eye/>
                                        {/if}
                                    </InputGroupButton>
                                </InputGroupAddon>
                            </InputGroup>
                        </Field>
                        <Field>
                            <FieldLabel for="confirm-password">确认密码</FieldLabel>
                            <InputGroup>
                                <InputGroupInput type={form.showConfirm ? 'text' : 'password'} placeholder="再次输入"
                                                 bind:value={form.confirm} name="confirm-password"
                                                 autocomplete="new-password"/>
                                <InputGroupAddon align="inline-end">
                                    <InputGroupButton size="icon-xs"
                                                      onclick={() => (form.showConfirm = !form.showConfirm)}
                                                      title={form.showConfirm ? '隐藏密码' : '显示密码'}
                                                      aria-label={form.showConfirm ? '隐藏密码' : '显示密码'}>
                                        {#if form.showConfirm}
                                            <EyeOff/>
                                        {:else}
                                            <Eye/>
                                        {/if}
                                    </InputGroupButton>
                                </InputGroupAddon>
                            </InputGroup>
                        </Field>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between text-xs text-muted-foreground">
                                <span>强度：{strength.label}</span>
                                <span>长度：{form.password.length}</span>
                            </div>
                            <div class="strength-meter" role="progressbar" aria-valuenow={strength.score}
                                 aria-valuemin="0"
                                 aria-valuemax="100">
                                {#each Array(5) as _, i}
                                    <div class="bar"
                                         data-active={i <= (Math.min(4, Math.floor(strength.score / 25)))}></div>
                                {/each}
                            </div>
                            <p class="text-xs text-muted-foreground">至少 12 个字符，包含大小写、数字与特殊字符</p>
                        </div>
                        <div class="grid gap-2">
                            <Button variant="ghost" size="sm" onclick={() => (form.advanced = !form.advanced)}
                                    aria-expanded={form.advanced}>显示高级选项
                            </Button>
                            {#if form.advanced}
                                <div class="grid gap-3">
                                    <div class="flex items-center gap-3">
                                        <span class="flex-none text-xs text-muted-foreground">长度</span>
                                        <Slider class="flex-auto" type="single" bind:value={sliderVal} max={64}
                                                step={1}/>
                                        <span class="flex-none text-xs text-muted-foreground">{sliderVal}</span>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <Button size="sm" onclick={() => genPassword(sliderVal)}>生成随机密码</Button>
                                        <Button variant="outline" size="sm"
                                                onclick={() => navigator.clipboard.writeText(form.password)}
                                                disabled={!form.password}>复制
                                        </Button>
                                    </div>
                                </div>
                            {/if}
                        </div>
                        {#if error}
                            <div class="text-destructive text-xs" role="alert" aria-live="assertive">{error}</div>
                        {/if}
                        {#if success}
                            <div class="text-green-600 text-xs" aria-live="polite">{success}</div>
                        {/if}

                        <div class="flex gap-2">
                            <Button class="flex-1" onclick={onSubmit} disabled={!canSubmit || loading}>
                                {#if loading}正在初始化...{:else}初始化{/if}
                            </Button>
                            <Button variant="outline" size="sm"
                                    onclick={() => { form.password = ''; form.confirm = ''; }}
                                    disabled={loading}>清空
                            </Button>

                        </div>
                    </FieldGroup>
                </div>


            </div>
        </div>

    </div>
    <div data-tauri-drag-region class="bg-muted relative hidden lg:block"
         style:background="url({randomLaunchImage()}) center/cover no-repeat">
    </div>
</div>

<style lang="scss">
  .strength-meter {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 6px
  }

  .strength-meter .bar {
    height: 8px;
    border-radius: var(--radius-sm);
    background-color: color-mix(in oklab, var(--color-muted) 80%, transparent)
  }

  .strength-meter .bar[data-active="true"] {
    background-color: var(--color-primary)
  }
</style>
