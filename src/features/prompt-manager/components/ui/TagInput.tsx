import { TextInput } from '@/components/ui/TextInput'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import IconButton from '@/components/ui/IconButton'

interface TagInputProps {
  tags: string[]
  setTags: (tags: string[]) => void
}

const TagInput: React.FC<TagInputProps> = ({ tags, setTags }) => {
  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    // if (event.key === 'Enter') としたいが、IME確定のEnterにも反応してしまうため、keyCodeで判定
    if (event.keyCode === 13) {
      event.preventDefault()
      const value = event.currentTarget.value.trim()
      addTag(value)
      event.currentTarget.value = ''
    }
  }

  const addTag = (tag: string) => {
    if (tag && !tags.includes(tag)) {
      setTags([...tags, tag])
    }
  }

  const removeTag = (index: number) => {
    setTags(tags.filter((_, i) => i !== index))
  }

  return (
    <div>
      <Label htmlFor="tags">Tags</Label>
      <TextInput
        id="tags"
        placeholder="Enter tags, separated by comma"
        type="text"
        onKeyDown={handleKeyDown}
      />
      <div className="flex flex-wrap gap-2 mt-2">
        {tags.map((tag: string, index: number) => (
          <Badge key={index} className="dark:bg-gray-600 dark:text-white">
            <span className="pr-1">{tag}</span>
            <IconButton
              icon="i-solar-close-circle-bold"
              onClick={() => {
                removeTag(index)
              }}
            />
          </Badge>
        ))}
      </div>
    </div>
  )
}

export default TagInput
