import { useEffect, useState } from 'react'
import { TextInput } from '@/components/ui/TextInput'
import { Label } from '@/components/ui/label'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'
import ButtonWithIcon from '@/components/ui/ButtonWithIcon'
import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { getPromptManagerAction, updatePromptManagerAction } from '../actions'
import { toast } from 'react-toastify'
import TagInput from './ui/TagInput'
import { useRecoilValue, useSetRecoilState } from 'recoil'
import { PromptManager } from '../types'
import { promptManagersAtom } from '@/store/atoms'

const formSchema = z.object({
  title: z.string().min(1).max(100),
  actionType: z.enum(['ComparingPrompt', 'ComparingModel']).optional(),
  apiType: z.enum(['Chat', 'Vision']).optional(),
})

interface PromptManagerEditFormProps {
  id: string
}

const PromptManagerEditForm: React.FC<PromptManagerEditFormProps> = ({
  id,
}) => {
  const promptManagers = useRecoilValue<PromptManager[]>(promptManagersAtom)
  const setPromptManagers = useSetRecoilState<PromptManager[]>(
    promptManagersAtom,
  )

  const [tags, setTags] = useState<string[]>([])

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: '',
      actionType: undefined,
      apiType: undefined,
    },
    shouldUnregister: false,
  })

  useEffect(() => {
    const fetchPromptManagers = async () => {
      try {
        const res = await getPromptManagerAction(Number(id))
        form.reset({
          title: res.title,
          actionType: res.actionType ?? undefined,
          apiType: res.apiType ?? undefined,
        })
        setTags(res.tags)
      } catch (error) {
        toast.error(`Failed to fetch prompt manager: ${error}`)
      }
    }
    fetchPromptManagers()
  }, [id])

  const updatePromptManager = async (data: z.infer<typeof formSchema>) => {
    try {
      await updatePromptManagerAction({
        id: Number(id),
        title: data.title,
        actionType: data.actionType,
        apiType: data.apiType,
        tags,
      })
      refreshPromptManagersState({
        id: Number(id),
        title: data.title,
        actionType: data.actionType ?? null,
        apiType: data.apiType ?? null,
        tags,
      })
      toast.info('Save Prompt Manager Success!')
    } catch (error) {
      toast.error(`Failed to update prompt manager: ${error}`)
    }
  }

  const refreshPromptManagersState = async (promptManager: PromptManager) => {
    // recoilのpromptManagersAtomで一致するIDのものを更新する
    const newPromptManagers = [...promptManagers]
    const index = newPromptManagers.findIndex((item) => item.id === Number(id))
    newPromptManagers[index] = promptManager
    setPromptManagers(newPromptManagers)
  }

  return (
    <div>
      <Form {...form}>
        <form
          className="flex flex-column gap-4"
          onSubmit={form.handleSubmit(updatePromptManager)}
        >
          <div className="grow w-7/12">
            <div className="mb-2">
              <FormField
                control={form.control}
                name="title"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Title</FormLabel>
                    <FormControl>
                      <TextInput
                        {...field}
                        placeholder="Enter title..."
                      ></TextInput>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <div className="flex gap-8">
              <FormField
                control={form.control}
                name="actionType"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Action Type</FormLabel>
                    <FormControl>
                      <RadioGroup
                        key={form.watch('actionType')}
                        onValueChange={field.onChange}
                        value={field.value}
                      >
                        <div className="flex row gap-4">
                          <div className="flex row items-center space-x-2">
                            <RadioGroupItem
                              value="ComparingPrompt"
                              id="ComparingPrompt"
                            />
                            <Label htmlFor="ComparingPrompt">
                              Comparing Prompt
                            </Label>
                          </div>
                          <div className="flex items-center space-x-2">
                            <RadioGroupItem
                              value="ComparingModel"
                              id="ComparingModel"
                            />
                            <Label htmlFor="ComparingModel">
                              Comparing Model
                            </Label>
                          </div>
                        </div>
                      </RadioGroup>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="apiType"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>API Type</FormLabel>
                    <FormControl>
                      <RadioGroup
                        key={form.watch('apiType')}
                        onValueChange={field.onChange}
                        value={field.value}
                      >
                        <div className="flex row gap-4">
                          <div className="flex row items-center space-x-2">
                            <RadioGroupItem value="Chat" id="Chat" />
                            <Label htmlFor="Chat">Chat</Label>
                          </div>
                          <div className="flex items-center space-x-2">
                            <RadioGroupItem value="Vision" id="Vision" />
                            <Label htmlFor="Vision">Vision</Label>
                          </div>
                        </div>
                      </RadioGroup>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
          </div>

          <div className="grow w-4/12">
            <TagInput tags={tags} setTags={setTags} />
          </div>

          <div className="flex items-center w-1/12">
            <ButtonWithIcon
              text="Save"
              type="submit"
              icon="i-solar-add-folder-bold"
              color="info"
              className="text-lg px-3 h-1/3"
            />
          </div>
        </form>
      </Form>
    </div>
  )
}

export default PromptManagerEditForm
