export interface SideMenuProps {
  onClickConfig: () => void
  onClickAdd: () => void
}

const SideMenuHeader = ({ onClickConfig, onClickAdd }: SideMenuProps) => {
  // TODO filterの追加、削除したものの表示をするようにする
  return (
    <div className="flex flex-row justify-between">
      <span className="font-bold text-lg">Propmtoty</span>
      <div>
        <span
          className="i-solar-add-circle-linear cursor-pointer"
          onClick={onClickAdd}
        ></span>
        <span
          className="i-solar-settings-bold cursor-pointer"
          onClick={onClickConfig}
        ></span>
      </div>
    </div>
  )
}

export default SideMenuHeader
