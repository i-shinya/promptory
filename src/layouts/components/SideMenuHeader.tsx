import IconButton from '../../components/ui/IconButton'

export interface SideMenuProps {
  onClickConfig: () => void
  onClickAdd: () => void
}

const SideMenuHeader = ({ onClickConfig, onClickAdd }: SideMenuProps) => {
  // TODO filterの追加、削除したものの表示をするようにする
  return (
    <div className="flex flex-row justify-between items-center">
      <span className="font-bold text-lg">Promptory</span>
      <div className="flex flex-row gap-px items-center">
        <IconButton icon="i-solar-add-circle-linear" onClick={onClickAdd} />
        <IconButton icon="i-solar-settings-bold" onClick={onClickConfig} />
      </div>
    </div>
  )
}

export default SideMenuHeader
