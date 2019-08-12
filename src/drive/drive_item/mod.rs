pub mod application;
pub mod asyncjobstatus;
pub mod audio;
pub mod baseitem;
pub mod baseitemversion;
pub mod calculatedcolumn;
pub mod choicecolumn;
pub mod collection;
pub mod columndefinition;
pub mod columnlink;
pub mod commentaction;
pub mod contenttype;
pub mod contenttypeinfo;
pub mod contenttypeorder;
pub mod createdby;
pub mod currencycolumn;
pub mod datetimecolumn;
pub mod defaultcolumnvalue;
pub mod deleteaction;
pub mod deleted;
pub mod drive;
pub mod driveinfo;
pub mod driveitem;
pub mod driveitemversion;
pub mod driverecipient;
pub mod expandchildren;
pub mod fieldvalueset;
pub mod file;
pub mod filesysteminfo;
pub mod folder;
pub mod folderview;
pub mod geocorrdinates;
pub mod group;
pub mod hashes;
pub mod identity;
pub mod identityset;
pub mod image;
pub mod itemactionset;
pub mod itemactivity;
pub mod itemactivityset;
pub mod itemactivitytimeset;
pub mod itemreference;
pub mod lastmodifiedby;
pub mod list;
pub mod listinfo;
pub mod listitem;
pub mod listitemversion;
pub mod lookupcolumn;
pub mod mentionaction;
pub mod moveaction;
pub mod numbercolumn;
pub mod owner;
pub mod package;
pub mod permissions;
pub mod personorgroupcolumn;
pub mod photo;
pub mod publicationfacet;
pub mod quota;
pub mod remoteitem;
pub mod renameaction;
pub mod searchresult;
pub mod shareaction;
pub mod shared;
pub mod shareddriveitem;
pub mod sharepointid;
pub mod sharinginvitation;
pub mod sharinglink;
pub mod site;
pub mod sitecollection;
pub mod specialfolder;
pub mod subscription;
pub mod textcolumn;
pub mod thumbnail;
pub mod uploadsession;
pub mod user;
pub mod versionaction;
pub mod video;
pub mod view;
pub mod webhooknotification;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Root {}
