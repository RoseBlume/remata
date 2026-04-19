# Lnk Main
<table class="frame"><tbody><tr><td>
<table class="inner" cellspacing="1">
<tbody><tr class="h"><th>Index1</th><th>Tag Name</th>
<th>Writable</th><th>Values / <span class="n">Notes</span></th></tr>
<tr>
<td title="0x0014 = 20">0x0014</td>
<td>Flags</td>
<td class="c">no</td>
<td><table class="cols"><tbody><tr>
  <td>Bit 0 = IDList
  <br>Bit 1 = LinkInfo
  <br>Bit 2 = Description
  <br>Bit 3 = RelativePath
  <br>Bit 4 = WorkingDir
  <br>Bit 5 = CommandArgs
  <br>Bit 6 = IconFile
  <br>Bit 7 = Unicode
  <br>Bit 8 = NoLinkInfo
  <br>Bit 9 = ExpString
  <br>Bit 10 = SeparateProc
  <br>Bit 12 = DarwinID
  <br>Bit 13 = RunAsUser
  <br>Bit 14 = ExpIcon
  <br>Bit 15 = NoPidAlias
  <br>Bit 17 = RunWithShim
  <br>Bit 18 = NoLinkTrack
  <br>Bit 19 = TargetMetadata
  <br>Bit 20 = NoLinkPathTracking
  <br>Bit 21 = NoKnownFolderTracking
  <br>Bit 22 = NoKnownFolderAlias
  <br>Bit 23 = LinkToLink
  <br>Bit 24 = UnaliasOnSave
  <br>Bit 25 = PreferEnvPath
  <br>Bit 26 = KeepLocalIDList</td></tr></tbody></table>
</td></tr>
<tr class="b">
<td title="0x0018 = 24">0x0018</td>
<td>FileAttributes</td>
<td class="c">no</td>
<td>--&gt; <a href="LNK.html#FileAttributes">LNK FileAttributes Values</a></td></tr>
<tr>
<td title="0x001c = 28">0x001c</td>
<td>CreateDate</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr class="b">
<td title="0x0024 = 36">0x0024</td>
<td>AccessDate</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr>
<td title="0x002c = 44">0x002c</td>
<td>ModifyDate</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr class="b">
<td title="0x0034 = 52">0x0034</td>
<td>TargetFileSize</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr>
<td title="0x0038 = 56">0x0038</td>
<td>IconIndex</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr class="b">
<td title="0x003c = 60">0x003c</td>
<td>RunWindow</td>
<td class="c">no</td>
<td><table class="cols"><tbody><tr>
  <td>0 = Hide
  <br>1 = Normal
  <br>2 = Show Minimized
  <br>3 = Show Maximized
  <br>4 = Show No Activate
  <br>5 = Show
  <br>6 = Minimized
  <br>7 = Show Minimized No Activate
  <br>8 = Show NA
  <br>9 = Restore
  <br>10 = Show Default</td></tr></tbody></table>
</td></tr>
<tr>
<td title="0x0040 = 64">0x0040</td>
<td>HotKey</td>
<td class="c">no</td>
<td><table class="cols"><tbody><tr>
  <td>0x0 = (none)
  <br>0x90 = Num Lock
  <br>0x91 = Scroll Lock
  <br>0x100 = Shift
  <br>0x200 = Control</td><td>&nbsp;&nbsp;</td>
  <td>0x400 = Alt
  <br>'0x30'-'0x39' = 0-9
  <br>'0x41'-'0x5a' = A-Z
  <br>'0x70'-'0x87' = F1-F24</td></tr></tbody></table>
</td></tr>
<tr class="b">
<td title="0x10000 = 65536">0x10000</td>
<td>ItemID</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#ItemID">LNK ItemID Tags</a></td></tr>
<tr>
<td title="0x20000 = 131072">0x20000</td>
<td>LinkInfo</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#LinkInfo">LNK LinkInfo Tags</a></td></tr>
<tr class="b">
<td title="0x30004 = 196612">0x30004</td>
<td>Description</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr>
<td title="0x30008 = 196616">0x30008</td>
<td>RelativePath</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr class="b">
<td title="0x30010 = 196624">0x30010</td>
<td>WorkingDirectory</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr>
<td title="0x30020 = 196640">0x30020</td>
<td>CommandLineArguments</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr class="b">
<td title="0x30040 = 196672">0x30040</td>
<td>IconFileName</td>
<td class="c">no</td>
<td>&nbsp;</td></tr>
<tr>
<td title="0xa0000000 = 2684354560">0xa0000000</td>
<td>UnknownData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr class="b">
<td title="0xa0000001 = 2684354561">0xa0000001</td>
<td>EnvVarData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#EnvVarData">LNK EnvVarData Tags</a></td></tr>
<tr>
<td title="0xa0000002 = 2684354562">0xa0000002</td>
<td>ConsoleData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#ConsoleData">LNK ConsoleData Tags</a></td></tr>
<tr class="b">
<td title="0xa0000003 = 2684354563">0xa0000003</td>
<td>TrackerData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#TrackerData">LNK TrackerData Tags</a></td></tr>
<tr>
<td title="0xa0000004 = 2684354564">0xa0000004</td>
<td>ConsoleFEData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#ConsoleFEData">LNK ConsoleFEData Tags</a></td></tr>
<tr class="b">
<td title="0xa0000005 = 2684354565">0xa0000005</td>
<td>SpecialFolderData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr>
<td title="0xa0000006 = 2684354566">0xa0000006</td>
<td>DarwinData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr class="b">
<td title="0xa0000007 = 2684354567">0xa0000007</td>
<td>IconEnvData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr>
<td title="0xa0000008 = 2684354568">0xa0000008</td>
<td>ShimData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr class="b">
<td title="0xa0000009 = 2684354569">0xa0000009</td>
<td>PropertyStoreData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr>
<td title="0xa000000b = 2684354571">0xa000000b</td>
<td>KnownFolderData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
<tr class="b">
<td title="0xa000000c = 2684354572">0xa000000c</td>
<td>VistaIDListData</td>
<td class="c">-</td>
<td>--&gt; <a href="LNK.html#UnknownData">LNK UnknownData Tags</a></td></tr>
</tbody></table></td></tr></tbody></table>

# File Attributes

<table class="frame"><tbody><tr><td>
<table class="inner sep" cellspacing="1">
<tbody><tr class="h"><th>Value</th><th>FileAttributes</th><th>Value</th><th>FileAttributes</th><th>Value</th><th>FileAttributes</th></tr>
<tr><td>'Bit 0'</td><td>= Read-only</td>
<td class="b">'Bit 5'</td><td class="b">= Archive</td>
<td>'Bit 10'</td><td>= Reparse point</td>
</tr><tr><td>'Bit 1'</td><td>= Hidden</td>
<td class="b">'Bit 6'</td><td class="b">= Encrypted?</td>
<td>'Bit 11'</td><td>= Compressed</td>
</tr><tr><td>'Bit 2'</td><td>= System</td>
<td class="b">'Bit 7'</td><td class="b">= Normal</td>
<td>'Bit 12'</td><td>= Offline</td>
</tr><tr><td>'Bit 3'</td><td>= Volume</td>
<td class="b">'Bit 8'</td><td class="b">= Temporary</td>
<td>'Bit 13'</td><td>= Not indexed</td>
</tr><tr><td>'Bit 4'</td><td>= Directory</td>
<td class="b">'Bit 9'</td><td class="b">= Sparse</td>
<td>'Bit 14'</td><td>= Encrypted</td>
</tr></tbody></table></td></tr></tbody></table>