import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { TextInput } from '@/components/ui/TextInput'
import { NumberInput } from '@/components/ui/NumberInput'
import ButtonWithIcon from '@/components/ui/ButtonWithIcon'
import { Textarea } from '@/components/ui/textarea'
import { z } from 'zod'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { RunSettings } from '../../types'

interface RunSettingEditFormProps {
  managerId: string
  runAction: (setting: RunSettings) => Promise<void>
}

const formSchema = z.object({
  userPrompt: z.string().min(1).max(10000),
  model: z.string().min(1).max(100),
  modelProvider: z.string().min(1).max(100),
  // input type=numberは文字列として値を持つが、coerceで数値に変換するっぽい
  temperature: z.coerce.number().min(0).max(1),
})

const RunSettingEditForm: React.FC<RunSettingEditFormProps> = ({
  managerId,
  runAction,
}) => {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      userPrompt: '',
      modelProvider: 'openai',
      model: 'gpt-4-1106-preview',
      temperature: 0,
    },
    shouldUnregister: false,
  })

  const run = async (data: z.infer<typeof formSchema>) => {
    console.log(data)
    const req = {
      ...data,
      managerId: Number(managerId),
    }
    await runAction(req)
  }

  return (
    <div className="flex flex-col h-full border-l border-solid border-zinc-400 pl-4 overflow-y-auto">
      <Form {...form}>
        <form
          className="flex flex-col gap-2 grow overflow-y-auto"
          onSubmit={form.handleSubmit(run)}
        >
          <FormField
            control={form.control}
            name="modelProvider"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Model Provider</FormLabel>
                <FormControl>
                  <TextInput {...field} placeholder="Enter Model." />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="model"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Model</FormLabel>
                <FormControl>
                  <TextInput {...field} placeholder="Enter Model." />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="temperature"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Temperature</FormLabel>
                <FormControl>
                  <NumberInput {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* <div> */}
          <div className="grow">
            <FormField
              control={form.control}
              name="userPrompt"
              render={({ field }) => (
                <FormItem className="flex flex-col h-full">
                  <FormLabel>User Prompt</FormLabel>
                  <FormControl>
                    <div className="dark:bg-neutral-600 p-1 h-full">
                      <Textarea
                        className="h-full"
                        {...field}
                        placeholder="Enter User Prompt."
                      />
                    </div>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>

          <div className="flex justify-evenly">
            <ButtonWithIcon
              text="Save"
              type="submit"
              icon="i-solar-add-folder-bold"
              color="info"
              className="text-2xl px-12"
            />
            <ButtonWithIcon
              text="Save & Run"
              type="submit"
              icon="i-solar-play-bold"
              color="success"
              className="text-2xl px-12"
            />
          </div>
        </form>
      </Form>
    </div>
  )
}

export default RunSettingEditForm
