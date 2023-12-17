import { useEffect } from 'react'
import { TextInput } from '@/components/ui/TextInput'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
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
      } catch (error) {
        toast.error(`Failed to fetch prompt manager: ${error}`)
      }
    }
    fetchPromptManagers()
  }, [id])

  const updatePromptManager = async (data: z.infer<typeof formSchema>) => {
    try {
      const tags: string[] = [] // TODO 画面で入力できるように修正
      await updatePromptManagerAction({
        id: Number(id),
        title: data.title,
        actionType: data.actionType,
        apiType: data.apiType,
        tags,
      })
      toast.info('Save Prompt Manager Success!')
    } catch (error) {
      toast.error(`Failed to update prompt manager: ${error}`)
    }
  }

  return (
    <div>
      <Form {...form}>
        <form
          className="flex flex-column gap-4"
          onSubmit={form.handleSubmit(updatePromptManager)}
        >
          <div className="grow">
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

          <div className="grow">
            <Label htmlFor="tags">Tags</Label>
            <TextInput
              id="tags"
              placeholder="Enter tags, separated by comma"
              type="text"
            />
            <div className="flex flex-wrap gap-2 mt-2">
              <Badge className="dark:bg-gray-600 dark:text-white">Tag1</Badge>
              <Badge className="dark:bg-gray-600 dark:text-white">Tag2</Badge>
            </div>
          </div>

          <div className="flex items-center">
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
