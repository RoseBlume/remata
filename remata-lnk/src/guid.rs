use remta_macros::{FromGuid, DisplayPretty};

#[derive(FromGuid, DisplayPretty)]
pub enum GUID {
    #[guid = "00021401-0000-0000-C000-000000000046"]
    ShellLinkClassIdentifier,
    #[guid = "0085E0A6-8D34-45D7-BC5C-447E59C73D48"]
    ContentTypeGenericFile,
    #[guid = "008CA0B1-55B4-4C56-B8A8-4DE4B299D3BE"]
    AccountPictures,
    #[guid = "00BCFC5A-ED94-4E48-96A1-3F6217F21990"]
    RoamingTiles,
    #[guid = "00F0C3AC-A593-49AC-9219-24ABCA5A2563"]
    ContentTypeMixedContentAlbum,
    #[guid = "012B0DB7-D4C1-45D6-B081-94B87779614F"]
    ContentTypeVideoAlbum,
    #[guid = "0139D44E-6AFE-49F2-8690-3DAFCAE6FFB8"]
    ProgramsCommon,
    #[guid = "031DA7EE-18C8-4205-847E-89A11261D0F3"]
    ContentTypeNetworkAssociation,
    #[guid = "031E4825-7B94-4DC3-B131-E946B44C8DD5"]
    Libraries,
    #[guid = "04731B67-D933-450A-90E6-4ACD2E9408FE"]
    SearchFolder,
    #[guid = "0482AF6C-08F1-4C34-8C90-E17EC98B1E17"]
    PublicAccountPicturesCommon,
    #[guid = "054FAE61-4DD8-4787-80B6-090220C4B700"]
    GameExplorerPerUser,
    #[guid = "0762D272-C50A-4BB0-A382-697DCD729B80"]
    UsersFixed,
    #[guid = "0AC0837C-BBF8-452A-850D-79D08E667CA7"]
    ComputerVirtual,
    #[guid = "0BAC070A-9F5F-4DA4-A8F6-3DE44D68FD6C"]
    ContentTypeWirelessProfile,
    #[guid = "0C39A5CF-1A7A-40C8-BA74-8900E6DF5FCD"]
    RecentItems,
    #[guid = "0D4C3DB6-03A3-462F-A0E6-08924C41B5D4"]
    HistoryPerUser,
    #[guid = "0F214138-B1D3-4A90-BBA9-27CBC0C5389A"]
    SyncSetupVirtual,
    #[guid = "0FED060E-8793-4B1E-90C9-48AC389AC631"]
    ContentTypeAppointment,
    #[guid = "15CA69B3-30EE-49C1-ACE1-6B5EC372AFB5"]
    SamplePlaylistsCommon,
    #[guid = "1777F761-68AD-4D8A-87BD-30B759FA33DD"]
    FavoritesPerUser,
    #[guid = "17789161-0268-45B3-8557-013009765873"]
    LocalAppData,
    #[guid = "18989B1D-99B5-455B-841C-AB7C74E4DDFC"]
    MyVideosPerUser,
    #[guid = "190337D1-B8CA-4121-A639-6D472D16972A"]
    SearchResultsVirtual,
    #[guid = "1A33F7E4-AF13-48F5-994E-77369DFE04A3"]
    ContentTypePlaylist,
    #[guid = "1A6FDBA2-F42D-4358-A798-B74D745926C5"]
    RecordedTVCommon,
    #[guid = "1AC14E77-02E7-4E5D-B744-2EB1AE5198B7"]
    System32Fixed,
    #[guid = "1B3EA5DC-B587-4786-B4EF-BD1DC332AEAE"]
    LibrariesPerUser,
    #[guid = "1E87508D-89C2-42F0-8A7E-645A0F50CA58"]
    ApplicationsVirtual,
    #[guid = "20D04FE0-3AEA-1069-A2D8-08002B30309D"]
    MyComputer,
    #[guid = "2112AB0A-C86A-4FFE-A368-0DE96E47012E"]
    MusicPerUser,
    #[guid = "21EC2020-3AEA-1069-A2DD-08002B30309D"]
    ControlPanelItems,
    #[guid = "22877A6D-37A1-461A-91B0-DBDA5AAEBC99"]
    RecentItemsAnother,
    #[guid = "2400183A-6185-49FB-A2D8-4A392A602BA3"]
    PublicVideosCommon,
    #[guid = "24D89E24-2F19-4534-9DDE-6A6671FBB8FE"]
    DocumentsPerUser,
    #[guid = "2559A1F1-21D7-11D4-BDAF-00C04F60B9F0"]
    WindowsHelpAndSupport,
    #[guid = "2559A1F3-21D7-11D4-BDAF-00C04F60B9F0"]
    RunDialogBox,
    #[guid = "26EE0668-A00A-44D7-9371-BEB064C98683"]
    ControlPanel,
    #[guid = "27E2E392-A111-48E0-AB0C-E17705A05F85"]
    ContentTypeFolder,
    #[guid = "289A9A43-BE44-4057-A41B-587A76D7E7F9"]
    SyncResultsVirtual,
    #[guid = "289af617-1cc3-42a6-926c-e6a863f0e3ba"]
    MyComputerAnother,
    #[guid = "28D8D31E-249C-454E-AABC-34883168E634"]
    ContentTypeUnspecified,
    #[guid = "2A00375E-224C-49DE-B8D1-440DF7EF3DDC"]
    NoneFixed,
    #[guid = "2B0F765D-C0E9-4171-908E-08A611B84FF6"]
    CookiesPerUser,
    #[guid = "2C36C0AA-5812-4B87-BFD0-4CD0DFB19B39"]
    OriginalImagesPerUser,
    #[guid = "3080F90D-D7AD-11D9-BD98-0000947B0257"]
    Desktop,
    #[guid = "3080F90E-D7AD-11D9-BD98-0000947B0257"]
    TaskView,
    #[guid = "3134ef9c-6b18-4996-ad04-ed5912e00eb5"]
    RecentFiles,
    #[guid = "31C0DD25-9439-4F12-BF41-7FF4EDA38722"]
    Objects3DPerUser,
    #[guid = "3214FAB5-9757-4298-BB61-92A9DEAA44FF"]
    PublicMusicCommon,
    #[guid = "339719B5-8C47-4894-94C2-D8F77ADD44A6"]
    PicturesPerUser,
    #[guid = "33E28130-4E1E-4676-835A-98395C3BC3BB"]
    PicturesPerUserAnother,
    #[guid = "346B8932-4C36-40D8-9415-1828291F9DE9"]
    ContentTypeContactGroup,
    #[guid = "352481E8-33BE-4251-BA85-6007CAEDCF9D"]
    TemporaryInternetFilesPerUser,
    #[guid = "35786d3c-b075-49b9-88dd-029876e11c01"]
    PortableDevices,
    #[guid = "374DE290-123F-4565-9164-39C4925E467B"]
    DownloadsPerUser,
    #[guid = "3936e9e4-d92c-4eee-a85a-bc16d5ea0819"]
    FrequentPlaces,
    #[guid = "3ADD1653-EB32-4CB0-BBD7-DFA0ABB5ACCA"]
    MyPictures,
    #[guid = "3B193882-D3AD-4EAB-965A-69829D1FB59F"]
    SavedPicturesPerUser,
    #[guid = "3D644C9B-1FB8-4F30-9B45-F670235F79C0"]
    PublicDownloadsCommon,
    #[guid = "3EB685DB-65F9-4CF6-A03A-E3EF65729F3D"]
    RoamingPerUser,
    #[guid = "4234D49B-0245-4DF3-B780-3893943456E1"]
    UserMusic,
    #[guid = "4336A54D-038B-4685-AB02-99BB52D3FB8B"]
    PublicUserRootFolder,
    #[guid = "43668BF8-C14E-49B2-97C9-747784D784B7"]
    SyncCenterVirtual,
    #[guid = "450D8FBA-AD25-11D0-A2A8-0800361B3003"]
    MyDocuments,
    #[guid = "48DAF80B-E6CF-4F4E-B800-0E69D84EE384"]
    LibrariesCommon,
    #[guid = "491E922F-5643-4AF4-A7EB-4E7A138D8174"]
    VideosPerUser,
    #[guid = "4AD2C85E-5E2D-45E5-8864-4F229E3C6CF0"]
    ContentTypeAudio,
    #[guid = "4BD8D571-6D19-48D3-BE97-422220080E43"]
    MusicPerUserAnother,
    #[guid = "4BFEFB45-347D-4006-A5BE-AC0CB0567192"]
    ConflictsVirtual,
    #[guid = "4C5C32FF-BB9D-43B0-B5B4-2D72E54EAAA4"]
    SavedGamesPerUser,
    #[guid = "4D9F7874-4E0C-4904-967B-40B0D20C3E4B"]
    TheInternetVirtual,
    #[guid = "52205FD8-5DFB-447D-801A-D0B52F2E83E1"]
    UserFiles,
    #[guid = "52A4F021-7B75-48A9-9F6B-4B87A210BC8F"]
    QuickLaunchPerUser,
    #[guid = "5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0"]
    ControlPanelAnother,
    #[guid = "53F5630D-B6BF-11D0-94F2-00A0C91EFB8B"]
    DeviceClassGuidVolume,
    #[guid = "559D40A3-A036-40FA-AF61-84CB430A4D34"]
    AppDataProgramDataPerUser,
    #[guid = "56784854-C6CB-462B-8169-88E350ACB882"]
    ContactsPerUser,
    #[guid = "59031A47-3F72-44A7-89C5-5595FE6B30EE"]
    UserProfile,
    #[guid = "5CD7AEE2-2219-4A67-B85D-6C9CE15660CB"]
    ProgramsPerUser,
    #[guid = "5CE4A5E9-E4EB-479D-B89F-130C02886155"]
    DeviceMetadataStoreCommon,
    #[guid = "5E591A74-DF96-48D3-8D67-1733BCEE28BA"]
    DelegateGuid,
    #[guid = "5E6C858F-0E22-4760-9AFE-EA3317B67173"]
    UsernameFixed,
    #[guid = "5E88B3CC-3E65-4E62-BFFF-229495253AB0"]
    ContentTypeMediaCast,
    #[guid = "60A169CF-F2AE-4E21-9375-9677F11C1C6E"]
    ContentTypeTelevision,
    #[guid = "625B53C3-AB48-4EC1-BA1F-A1EF4146FC19"]
    StartMenuPerUser,
    #[guid = "62AB5D82-FDC1-4DC3-A9DD-070D1D495D97"]
    ProgramDataFixed,
    #[guid = "63252F2C-887F-4CB6-B1AC-D29855DCEF6C"]
    ContentTypeTask,
    #[guid = "6365D5A7-0F0D-45E5-87F6-0DA56B6A4F7D"]
    CommonFilesFixed,
    #[guid = "640167b4-59b0-47a6-b335-a6b3c0695aea"]
    PortableMediaDevices,
    #[guid = "680ADF52-950A-4041-9B41-65E393648155"]
    ContentTypeDocument,
    #[guid = "69D2CF90-FC33-4FB7-9A0C-EBB0F0FCB43C"]
    SlideshowsPerUser,
    #[guid = "6D809377-6AF0-444B-8957-A3773F02200E"]
    ProgramFilesFixed,
    #[guid = "6F0CD92B-2E97-45D1-88FF-B0D186B8DEDD"]
    NetworkConnectionsVirtual,
    #[guid = "7007ACC7-3202-11D1-AAD2-00805FC1270E"]
    NetworkConnections,
    #[guid = "724EF170-A42D-4FEF-9F26-B60E846FBA4F"]
    AdministrativeToolsPerUser,
    #[guid = "75793148-15F5-4A30-A813-54ED8A37E226"]
    ContentTypeImageAlbum,
    #[guid = "767E6811-49CB-4273-87C2-20F355E1085B"]
    CameraRollPerUser,
    #[guid = "76FC4E2D-D6AD-4519-A663-37BD56068185"]
    PrintersVirtual,
    #[guid = "7B0DB17D-9CD2-4A93-9733-46CC89022E7C"]
    DocumentsPerUserAnother,
    #[guid = "7B396E54-9EC5-4300-BE0A-2482EBAE1A26"]
    GadgetsCommon,
    #[guid = "7BE16610-1F7F-44AC-BFF0-83E15F2FFCA1"]
    AppDataDocumentsPerUser,
    #[guid = "7C5A40EF-A0FB-4BFC-874A-C0F2E0B9FA8E"]
    ProgramFilesFixedAnother,
    #[guid = "7CFBEFBC-DE1F-45AA-B843-A542AC536CC9"]
    AppDataFavoritesPerUser,
    #[guid = "7D1D3A04-DEBB-4115-95CF-2F29DA2920DA"]
    SearchesPerUser,
    #[guid = "7E636BFE-DFA9-4D5E-B456-D7B39851D8A9"]
    TemplatesPerUser,
    #[guid = "8038044A-7E51-4F8F-883D-1D0623D14533"]
    ContentTypeEmail,
    #[guid = "80E170D2-1055-4A3E-B952-82CC4F8A8689"]
    ContentTypeAll,
    #[guid = "821089F5-1D91-4DC9-BE3C-BBB1B35B18CE"]
    ContentTypeSection,
    #[guid = "82A5EA35-D9CD-47C5-9629-E15D2F714E6E"]
    StartupCommon,
    #[guid = "82A74AEB-AEB4-465C-A014-D097EE346D63"]
    ControlPanelVirtual,
    #[guid = "859EAD94-2E85-48AD-A71A-0969CB56A6CD"]
    SampleVideosCommon,
    #[guid = "871C5380-42A0-1069-A2EA-08002B30309D"]
    Internet,
    #[guid = "896664f7-12e1-490f-8782-c0835afd98fc"]
    LibrariesAnother,
    #[guid = "8983036C-27C0-404B-8F08-102D10DCFD74"]
    SendToPerUser,
    #[guid = "8AD10C31-2ADB-4296-A8F7-E4701232C972"]
    ResourcesFixed,
    #[guid = "905E63B6-C1BF-494E-B29C-65B732D3D21A"]
    ProgramFilesFixedAnother2,
    #[guid = "9113a02d-00a3-46b9-bc5f-9c04daddd5d7"]
    EnhancedStorageDataSource,
    #[guid = "9261B03C-3D78-4519-85E3-02C5E1F50BB9"]
    ContentTypeVideo,
    #[guid = "9274BD8D-CFD1-41C3-B35E-B13F55A758F4"]
    PrinterShortcutsPerUser,
    #[guid = "98EC0E18-2098-4D44-8644-66979315A281"]
    MicrosoftOfficeOutlookVirtual,
    #[guid = "99ED0160-17FF-4C44-9D98-1D7A6F941921"]
    ContentTypeFunctionalObject,
    #[guid = "9B74B6A3-0DFD-4F11-9E78-5F7800F2E772"]
    UsernameVirtual,
    #[guid = "9CD20ECF-3B50-414F-A641-E473FFE45751"]
    ContentTypeMemo,
    #[guid = "9E395ED8-512D-4315-9960-9110B74616C8"]
    RecentItemsAnother2,
    #[guid = "9E52AB10-F80D-49DF-ACB8-4330F5687855"]
    UserPinnedPerUser,
    #[guid = "9E3995AB-1F9C-4F13-B827-48B24B6C7174"]
    TemporaryBurnFolderPerUser,
    #[guid = "9db7a13c-f208-4981-8353-73cc61ae2783"]
    PreviousVersions,
    #[guid = "A1FD5967-6023-49A0-9DF1-F8060BE751B0"]
    ContentTypeCalendar,
    #[guid = "A302545D-DEFF-464B-ABE8-61C8648D939B"]
    LibrariesVirtual,
    #[guid = "A305CE99-F527-492B-8B1A-7E76FA98D6E4"]
    InstalledUpdatesVirtual,
    #[guid = "A3918781-E5F2-4890-B3D9-A7E54332328C"]
    ApplicationShortcutsPerUser,
    #[guid = "A4115719-D62E-491D-AA7C-E74B8BE3B067"]
    StartMenuCommon,
    #[guid = "A520A1A4-1780-4FF6-BD18-167343C5AF16"]
    LocalLowPerUser,
    #[guid = "A52BBA46-E9E1-435F-B3D9-28DAA648C0F6"]
    OneDrivePerUser,
    #[guid = "A63293E8-664E-48DB-A079-DF759E0509F7"]
    TemplatesPerUserAnother,
    #[guid = "A75D362E-50FC-4FB7-AC2C-A8BEAA314493"]
    GadgetsPerUser,
    #[guid = "A77F5D77-2E2B-44C3-A6A2-ABA601054A51"]
    ProgramsPerUserAnother,
    #[guid = "A8CDFF1C-4878-43BE-B5FD-F8091C1C60D0"]
    Documents,
    #[guid = "A990AE9F-A03B-4E80-94BC-9912D7504104"]
    PicturesPerUserAnother2,
    #[guid = "AA18737E-5009-48FA-AE21-85F24383B4E6"]
    ContentTypeAudioAlbum,
    #[guid = "AAA8D5A5-F1D6-4259-BAA8-78E7EF60835E"]
    RoamedTileImagesPerUser,
    #[guid = "AB5FB87B-7CE2-4F83-915D-550846C9537B"]
    CameraRollPerUserAnother,
    #[guid = "AE50C081-EBD2-438A-8655-8A092E34987A"]
    RecentItemsPerUser,
    #[guid = "B250C668-F57D-4EE1-A63C-290EE7D1AA1F"]
    SampleMusicCommon,
    #[guid = "B2C5E279-7ADD-439F-B28C-C41FE1BBF672"]
    AppDataDesktopPerUser,
    #[guid = "B4BFCC3A-DB2C-424C-B029-7FE99A87C641"]
    DesktopAnother,
    #[guid = "B6EBFB86-6907-413C-9AF7-4FC2ABF07CC5"]
    PublicPicturesCommon,
    #[guid = "B7534046-3ECB-4C18-BE4E-64CD4CB7D6AC"]
    RecycleBinVirtual,
    #[guid = "B7BEDE81-DF94-4682-A7D8-57A52620B86F"]
    ScreenshotsPerUser,
    #[guid = "B94237E7-57AC-4347-9151-B08C6C32D1F7"]
    TemplatesCommon,
    #[guid = "B97D20BB-F46A-4C97-BA10-5E3608430854"]
    StartupPerUser,
    #[guid = "BCB5256F-79F6-4CEE-B725-DC34E402FD46"]
    ImplicitAppShortcutsPerUser,
    #[guid = "BCBD3057-CA5C-4622-B42D-BC56DB0AE516"]
    ProgramsPerUserAnother2,
    #[guid = "BFB9D5E0-C6A9-404C-B2B2-AE6DB6AF4968"]
    LinksPerUser,
    #[guid = "C1BAE2D0-10DF-4334-BEDD-7AA20B227A9D"]
    OEMLinksCommon,
    #[guid = "C4900540-2379-4C75-844B-64E6FAF8716B"]
    SamplePicturesCommon,
    #[guid = "C4AA340D-F20F-4863-AFEF-F87EF2E6BA25"]
    PublicDesktopCommon,
    #[guid = "C5ABBF53-E17F-4121-8900-86626FC2C973"]
    NetworkShortcutsPerUser,
    #[guid = "C870044B-F49E-4126-A9C3-B52A1FF411E8"]
    RingtonesPerUser,
    #[guid = "CAC52C1A-B53D-4EDC-92D7-6B2E8AC19434"]
    GamesVirtual,
    #[guid = "D0384E7D-BAC3-4797-8F14-CBA229B392B5"]
    AdministrativeToolsCommon,
    #[guid = "D20BEEC4-5CA8-4905-AE3B-BF251EA09B53"]
    NetworkVirtual,
    #[guid = "D269F96A-247C-4BFF-98FB-97F3C49220E6"]
    ContentTypeProgram,
    #[guid = "D3162B92-9365-467A-956B-92703ACA08AF"]
    UserDocuments,
    #[guid = "D65231B0-B2F1-4857-A4CE-A8E7C6EA7D27"]
    System32FixedAnother,
    #[guid = "D9DC8A3B-B784-432E-A781-5A1130A75963"]
    HistoryPerUserAnother,
    #[guid = "DC3876E8-A948-4060-9050-CBD77E8A3D87"]
    ContentTypeCertificate,
    #[guid = "DE61D971-5EBC-4F02-A3A9-6C82895E5C04"]
    GetProgramsVirtual,
    #[guid = "DE92C1C7-837F-4F69-A3BB-86E631204A23"]
    PlaylistsPerUser,
    #[guid = "DE974D24-D9C6-4D3E-BF91-F4455120B917"]
    CommonFilesFixedAnother,
    #[guid = "DEBF2536-E1A8-4C59-B6A2-414586476AEA"]
    GameExplorerCommon,
    #[guid = "DF7266AC-9274-4867-8D55-3BD661DE872D"]
    ProgramsAndFeaturesVirtual,
    #[guid = "DFDF76A2-C82A-4D63-906A-5644AC457385"]
    PublicFixed,
    #[guid = "DFFACDC5-679F-4156-8947-C5C76BC0B67F"]
    UsersFiles,
    #[guid = "E25B5812-BE88-4BD9-94B0-29233477B6C3"]
    SavedPicturesLibraryPerUser,
    #[guid = "E555AB60-153B-4D17-9F04-A5FE99FC15EC"]
    RingtonesCommon,
    #[guid = "E80EAAF8-B2DB-4133-B67E-1BEF4B4A6E5F"]
    ContentTypeGenericMessage,
    #[guid = "EABA8313-4525-4707-9F0E-87C6808E9435"]
    ContentTypeContact,
    #[guid = "ED228FDF-9EA8-4870-83B1-96B02CFE0D52"]
    GameExplorer,
    #[guid = "ED4824AF-DCE4-45A8-81E2-FC7965083634"]
    PublicDocumentsCommon,
    #[guid = "EE32E446-31CA-4ABA-814F-A5EBD2FD6D5E"]
    OfflineFilesVirtual,
    #[guid = "EF2107D5-A52A-4243-A26B-62D4176D7603"]
    ContentTypeImage,
    #[guid = "F1B32785-6FBA-4FCF-9D55-7B8E7F157091"]
    LocalPerUser,
    #[guid = "F3364BA0-65B9-11CE-A9BA-00AA004AE661"]
    SearchResultsFolder,
    #[guid = "F3CE0F7C-4901-4ACC-8648-D5D44B04EF8F"]
    UsersFullNameVirtual,
    #[guid = "F42EE2D3-909F-4907-8871-4C22FC0BF756"]
    DocumentsAnother,
    #[guid = "F7F1ED05-9F6D-47A2-AAAE-29D317C6F066"]
    CommonFilesFixedAnother2,
    #[guid = "FD228CB7-AE11-4AE3-864C-16F3910AB8FE"]
    FontsFixed,
    #[guid = "FDD39AD0-238F-46AF-ADB4-6C85480369C7"]
    DocumentsPerUserAnother2,
    #[guid = "b155bdf8-02f0-451e-9a26-ae317cfd7779"]
    NetHood,
    #[guid = "c2b136e2-d50e-405c-8784-363c582bf43e"]
    WirelessDevices,
    #[guid = "d34a6ca6-62c2-4c34-8a7c-14709c1ad938"]
    CommonPlaces,
    #[guid = "dffacdc5-679f-4156-8947-c5c76bc0b67f"]
    Profile,
    #[guid = "ed50fc29-b964-48a9-afb3-15ebb9b97f36"]
    PrintHood,
    #[guid = "f5fb2c77-0e2f-4a16-a381-3e560c68bc83"]
    RemovableDrives,
}