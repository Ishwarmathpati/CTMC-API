<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddShippingAddressId</name>
   <tag></tag>
   <elementGuidId>9d606b9c-0e2c-4f41-a717-40b508cbd5e2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${business-unit-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addShippingAddressId\&quot;,\n            \&quot;addressId\&quot; : \&quot;${addressId}\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>53055336-4b61-484b-8794-cce1c61be0b3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>19f9796d-e5b3-4b7c-b05d-ccb31227a1e8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/business-units/${business-unit-id}</restUrl>
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
      <id>c50d5a1a-9c20-4a7a-b207-a651bdad5b56</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>fc59a214-36ed-4648-8595-0bdcac4328b5</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-id</defaultValue>
      <description></description>
      <id>a0f89320-c0bd-4a3e-8d19-5fb155e6d251</id>
      <masked>false</masked>
      <name>business-unit-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-version</defaultValue>
      <description></description>
      <id>4d270145-bcee-4933-a03d-fc9235c37952</id>
      <masked>false</masked>
      <name>business-unit-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.addressId</defaultValue>
      <description></description>
      <id>17722194-4bfe-4f21-bc36-1262e14f27dc</id>
      <masked>false</masked>
      <name>addressId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
