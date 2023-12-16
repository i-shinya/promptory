import Card from '../../../../components/ui/Card'
import ButtonWithIcon from '../../../../components/ui/ButtonWithIcon'
import { TextInput } from '@/components/ui/TextInput'
import * as z from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from '@/components/ui/form'

export interface TemporaryPromptManagerCardProps {
  handleSubmit: (title: string) => Promise<void>
  onClickCancel: () => void
}

const formSchema = z.object({
  title: z.string().min(1).max(100),
})

const NewPromptManagerInputForm = ({
  handleSubmit,
  onClickCancel,
}: TemporaryPromptManagerCardProps) => {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: '',
    },
  })

  const onSubmit = async (data: z.infer<typeof formSchema>) =>
    await handleSubmit(data.title)

  return (
    <Card>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)}>
          <FormField
            control={form.control}
            name="title"
            render={({ field }) => (
              <FormItem>
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
          <div className="flex justify-end gap-2 mt-2">
            <ButtonWithIcon
              text="Cancel"
              type="button"
              icon="i-solar-close-circle-bold"
              color="warn"
              onClick={onClickCancel}
            />
            <ButtonWithIcon
              text="Save"
              type="submit"
              icon="i-solar-add-folder-bold"
              color="info"
            />
          </div>
        </form>
      </Form>
    </Card>
  )
}

export default NewPromptManagerInputForm
