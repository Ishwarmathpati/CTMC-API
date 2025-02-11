<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create customer in store</name>
   <tag></tag>
   <elementGuidId>1ecfd11f-358c-4771-93b7-71b9f3bbcdd4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot; : \&quot;johndoe@example.com\&quot;,\n  \&quot;firstName\&quot; : \&quot;John\&quot;,\n  \&quot;lastName\&quot; : \&quot;Doe\&quot;,\n  \&quot;password\&quot; : \&quot;secret123\&quot;\n}&quot;,
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
      <webElementGuid>edb10d3a-53fd-4483-b9b9-3ea2759631a0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>08157cf8-7808-4dbd-8923-d4a3bed0a91b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/customers</restUrl>
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
      <id>27196d45-e0c9-4fb8-b0ae-591e67873218</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>85c248d0-b184-48fe-9a87-9753560cdebf</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>6c3f5c71-4cc0-4b2d-ac07-68b7cabd62b2</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
