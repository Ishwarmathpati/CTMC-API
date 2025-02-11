<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCustomType</name>
   <tag></tag>
   <elementGuidId>82aef494-5099-4a0b-baff-5b6e86922a03</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${channel-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setCustomType\&quot;,\n            \&quot;type\&quot; : {\n              \&quot;id\&quot; : \&quot;${type-id}\&quot;,\n              \&quot;typeId\&quot; : \&quot;type\&quot;\n            },\n            \&quot;fields\&quot; : {\n              \&quot;exampleStringTypeField\&quot; : \&quot;TextString\&quot;\n            }\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>2ea1ee6e-0eb1-4823-a93b-cea9fc75c498</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2187a220-134e-47eb-a676-9b89fb3accc1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/channels/${channel-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>38af5f36-f8d7-4dd9-b15e-1c52de617501</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>5616e337-611e-4742-88d3-e19b8227beb7</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel-id</defaultValue>
      <description></description>
      <id>54bb8f65-d001-4d55-a641-fb3f58dbf4dd</id>
      <masked>false</masked>
      <name>channel-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel-version</defaultValue>
      <description></description>
      <id>471bf0ed-b948-49fe-8f7e-159a623c997a</id>
      <masked>false</masked>
      <name>channel-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type-id</defaultValue>
      <description></description>
      <id>73f9ceda-3597-448c-a81f-2e9d3089744f</id>
      <masked>false</masked>
      <name>type-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
