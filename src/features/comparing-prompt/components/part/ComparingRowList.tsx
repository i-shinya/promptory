import { ComparingPromtpRow } from '../../types'
import ComparingResultRow from '../ui/ComparingResultRow'

interface ComparingRowListProps {
  comparingRows: ComparingPromtpRow[]
  setComparingRows: (comparingRows: ComparingPromtpRow[]) => void
}

const ComparingRowList: React.FC<ComparingRowListProps> = ({
  comparingRows,
  setComparingRows,
}) => {
  // idが一致するrowのsystemPromptを更新する
  const setSytemPrompt = (id: number, systemPrompt: string) => {
    const newSystemPrompts = [...comparingRows]
    const index = newSystemPrompts.findIndex((item) => item.id === id)
    newSystemPrompts[index].systemPrompt = systemPrompt
    setComparingRows(newSystemPrompts)
  }

  const deleteRow = (id: number) => {
    const newSystemPrompts = comparingRows.filter((item) => item.id !== id)
    setComparingRows(newSystemPrompts)
  }

  return (
    <div className="flex flex-col gap-2">
      {comparingRows.length > 0 ? (
        <>
          {comparingRows.map((item) => (
            <ComparingResultRow
              key={item.id}
              answer={item.answer}
              systemPrompt={item.systemPrompt}
              setSystemPrompt={(systemPrompt) => {
                setSytemPrompt(item.id, systemPrompt)
              }}
              deleteRow={() => {
                deleteRow(item.id)
              }}
            />
          ))}
        </>
      ) : (
        <div>No Comparing Rows</div>
      )}
    </div>
  )
}

export default ComparingRowList
